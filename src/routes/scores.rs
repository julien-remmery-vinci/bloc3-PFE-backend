use axum::{extract::{ Path, State}, Json};

use crate::{database::state::AppState, errors::responserror::ResponseError};

#[axum::debug_handler]
pub async fn sum_score_template(
    State(state): State<AppState>,
    Path(form_id): Path<i32>,
) -> Result<Json<f64>, ResponseError> {
    let template=state.score.find_template_by_form_id(form_id).await?;
    println!("{:?}",template);
    let mut score_total = 0.0;
    // si template contient ALL 
    if template.contains(&"ALL".to_string()) {
        score_total = state.score.sum_score_template_all().await?;
    }
    else {
       for t in template {
        score_total += state.score.sum_score_template(t).await?;
        } 
    }
    
    println!("{:?}",score_total);
    let score_user_now = state.score.sum_score_user_now(form_id).await?;
    println!("{:?}",score_user_now);
    let score_user_commitment_pact = state.score.sum_score_user_commitment_pact(form_id).await?;
    println!("{:?}",score_user_commitment_pact);
    let score = (score_user_now + score_user_commitment_pact)/score_total;
    println!("{:?}",score);
    Ok(Json(score))
}