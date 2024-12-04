use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{
    database::state::AppState,
    services::questions::{self, QuestionError},
};

#[derive(Deserialize)]
pub struct QuestionRequest {
    pub question_status: String,
    pub category: String,
    pub sub_category: String,
    pub question: String,
}

#[derive(Serialize)]
pub struct OkResponse {
    id: i32,
}

pub async fn post_question(
    State(state): State<AppState>,
    Json(question): Json<QuestionRequest>,
) -> Result<Json<OkResponse>, crate::services::questions::QuestionError> {
    question.invalid()?;
    let id = state.question.create_question(question).await?;
    Ok(Json(OkResponse { id }))
}

impl QuestionRequest {
    pub fn invalid(&self) -> Result<(), QuestionError> {
        if self.question.is_empty()
            || self.question_status.is_empty()
            || self.category.is_empty()
            || self.sub_category.is_empty()
        {
            return Err(QuestionError::BadRequest);
        }
        Ok(())
    }
}
