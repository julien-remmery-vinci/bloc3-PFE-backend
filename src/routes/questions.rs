use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};

use crate::{
    database::state::AppState, errors::responserror::ResponseError, models::question::{PutQuestionRequest, Question, QuestionRequest},
};

pub async fn create_question(
    State(state): State<AppState>,
    Json(question): Json<QuestionRequest>,
) -> Result<impl IntoResponse, ResponseError> {
    question.validate()?;
    state.question.create_question(question).await?;
    Ok(StatusCode::CREATED)
}

pub async fn read_one_question(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Question>, ResponseError> {
    let question = state.question.read_one_question(id).await?;
    Ok(Json(question))
}

pub async fn update_question(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(question): Json<PutQuestionRequest>,
) -> Result<impl IntoResponse, ResponseError> {
    question.update_validate()?;
    state.question.update_question(id, question).await?;
    Ok(StatusCode::NO_CONTENT)
}