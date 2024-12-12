use axum::{extract::State, Json};

use crate::{database::state::AppState, errors::responserror::ResponseError, models::stats::Stats};

pub async fn get_stats(
    State(state): State<AppState>,
) -> Result<Json<Stats>, ResponseError> {
    let stats = state.stats.get_stats().await?;
    Ok(Json(stats))
}