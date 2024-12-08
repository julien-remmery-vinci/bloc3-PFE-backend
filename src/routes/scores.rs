use axum::{extract::{ Path, State}, Json};

use crate::{database::state::AppState, errors::responserror::ResponseError};

#[axum::debug_handler]
pub async fn sum_score_template(
    State(state): State<AppState>,
    Path(form_id): Path<i32>,
) -> Result<Json<f64>, ResponseError> {
    let template=state.score.find_template_by_form_id(form_id).await?;
    let score = state.score.sum_score_template(template).await?;
    Ok(Json(score))
}