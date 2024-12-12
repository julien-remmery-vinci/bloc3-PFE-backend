use axum::{extract::{ Path, State}, Extension, Json};

use crate::{database::state::AppState, errors::responserror::ResponseError, models::{score::Score, user::User}};

#[axum::debug_handler]
pub async fn sum_score_template(
    State(state): State<AppState>,
    Path(form_id): Path<i32>,
    Extension(user): Extension<User>,
) -> Result<Json<Score>, ResponseError> {
    if form_id < 1 {
        return Err(ResponseError::BadRequest(Some(String::from("Invalid form id"))));
    }
    match state.form.read_form_by_id(form_id).await? {
        Some(_) => (),
        None => return Err(ResponseError::NotFound(Some(String::from("Form not found")))),
    }
    /* 
    let template=state.score.find_template_by_form_id(form_id).await?;
    let mut score_total = 0.0;
    // si template est ALL 
    if template.contains(&"ALL".to_string()) {
        score_total = state.score.sum_score_template_all().await?;
    }
    else {
       for t in template {
        score_total += state.score.sum_score_template(t).await?;
        } 
        score_total += state.score.sum_score_template("ALL".to_string()).await?;
    }
    */
    let score_user_now = state.score.sum_score_user_now(form_id).await?;
    let mut sum: f64 = 0.0;
    let score_user_now_clone = score_user_now.clone();
    if let Some(scores) = score_user_now {
        for s in scores.iter() {
            if let Some(score) = s.score {
                sum += score;
            }
        }
    } else {
        println!("No scores available");
    }
    let score_user_commitment_pact = state.score.sum_score_user_commitment_pact(form_id).await?;
    let score_user_commitment_pact_clone = score_user_commitment_pact.clone();
    if let Some(scores) = score_user_commitment_pact {
        for s in scores.iter() {
            if let Some(score) = s.score {
                sum += score;
            }
        }
    } else {
        println!("No scores available");
    }
    //score en %
    let score = ((sum)/90.0)*100.0;
    
    if user.role != "admin" {
        return Ok(Json(Score {
            total: score,
            details_now: Vec::new(),
            details_commitment_pact: Vec::new()
        }))
    }

    Ok(Json(Score {
        total: score,
        details_now: score_user_now_clone.unwrap_or_default(),
        details_commitment_pact: score_user_commitment_pact_clone.unwrap_or_default()
    }))
}