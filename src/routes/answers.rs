use axum::{extract::State, Json};

use crate::models::{
    answer::Answer,
    create_answer::CreateAnswer,
};
use crate::database::state::AppState;
use crate::errors::answer_error::AnswerError;

pub async fn add_answer(
    State(state): State<AppState>,
    Json(answer): Json<CreateAnswer>,
) -> Result<Json<Answer>, AnswerError> {
    let valid = state.answer.add_answer(answer).await?;
    Ok(Json(valid))
}