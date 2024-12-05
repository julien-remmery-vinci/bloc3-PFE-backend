use axum::{debug_handler, extract::{Path, State}, Json};
use serde::{Deserialize, Serialize};

use crate::{
    database::state::AppState,
    errors::questionserror::QuestionError,
};

#[derive(Deserialize, Serialize)]
pub struct QuestionRequest {
    pub question_status: String,
    pub category: String,
    pub sub_category: String,
    pub question: String,
}

#[derive(Deserialize)]
pub struct PutQuestionRequest {
    pub question: String,
}

#[derive(Serialize)]
pub struct OkResponse {
    id: i32,
}

pub async fn create_question(
    State(state): State<AppState>,
    Json(question): Json<QuestionRequest>,
) -> Result<Json<OkResponse>, crate::errors::questionserror::QuestionError> {
    question.validate()?;
    let id = state.question.create_question(question).await?;
    Ok(Json(OkResponse { id }))
}

pub async fn read_one_question(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<QuestionRequest>, QuestionError> {
    let question = state.question.read_one_question(id).await?;
    Ok(Json(question))
}

pub async fn update_question(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(question): Json<PutQuestionRequest>,
) -> Result<Json<OkResponse>, QuestionError> {
    question.update_validate()?;
    state.question.update_question(id, question).await?;
    Ok(Json(OkResponse { id }))
}

impl QuestionRequest {
    pub fn validate(&self) -> Result<(), QuestionError> {
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

impl PutQuestionRequest {
    pub fn update_validate(&self) -> Result<(), QuestionError> {
        if self.question.is_empty() {
            return Err(QuestionError::BadRequest);
        }
        Ok(())
    }
}
