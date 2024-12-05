use axum::{extract::State, Json};
use serde::Deserialize;

use crate::{
    database::state::AppState,
    errors::answer_error::AnswerError,
    models::answers::Answer,
};

#[derive(Deserialize)]
pub struct CreateAnswer {
    pub answer: String,
    pub template: String,
    pub question_id: i32,
    pub score: f64,
    pub engagement_score: f64,
    pub is_forced_engagement: bool,
    pub comment: String,
}

#[axum::debug_handler]
pub async fn create_answer(
    State(state): State<AppState>,
    Json(answer): Json<CreateAnswer>,
) -> Result<Json<Answer>, AnswerError> {
    if answer.invalid() {
        return Err(AnswerError::BadRequest);
    }
    let valid = state.answer.create_answer(answer).await?;
    Ok(Json(valid))
}

impl CreateAnswer {
    pub fn invalid(&self) -> bool {
        self.answer.is_empty()
            || self.template.is_empty()
            || self.question_id == 0
            || self.score < 0.0
            || self.engagement_score < 0.0
    }
}