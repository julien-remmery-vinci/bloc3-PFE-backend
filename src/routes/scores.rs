use axum::{extract::{ Query, State}, Json};

use crate::{database::state::AppState, errors::score_error::ScoreError, models::score::ScoreQuery};

#[axum::debug_handler]
pub async fn sum_score_template(
    State(state): State<AppState>,
    Query(query): Query<ScoreQuery>,
) -> Result<Json<f64>, ScoreError> {
    let score = state.score.sum_score_template(query.form_id.to_string()).await?;
    Ok(Json(score))
}