use axum::{extract::{Path, State}, Json};
use serde::Serialize;

use crate::{
    database::state::AppState, errors::globalerror::ResponseError, models::question::{PutQuestionRequest, QuestionRequest},
};

#[derive(Serialize)]
pub struct OkResponse {
    id: i32,
}

pub async fn create_question(
    State(state): State<AppState>,
    Json(question): Json<QuestionRequest>,
) -> Result<Json<OkResponse>, ResponseError> {
    question.validate()?;
    let id = state.question.create_question(question).await?;
    Ok(Json(OkResponse { id }))
}

pub async fn read_one_question(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<QuestionRequest>, ResponseError> {
    let question = state.question.read_one_question(id).await?;
    Ok(Json(question))
}

pub async fn update_question(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(question): Json<PutQuestionRequest>,
) -> Result<Json<OkResponse>, ResponseError> {
    question.update_validate()?;
    state.question.update_question(id, question).await?;
    Ok(Json(OkResponse { id }))
}