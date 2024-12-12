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
    if id < 1 {
        return Err(ResponseError::BadRequest(Some(String::from("Invalid question id"))));
    }
    let question: Question = state.question.read_one_question(id).await?;
    Ok(Json(question))
}

pub async fn update_question(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(question): Json<PutQuestionRequest>,
) -> Result<impl IntoResponse, ResponseError> {
    if id < 1 {
        return Err(ResponseError::BadRequest(Some(String::from("Invalid question id"))));
    }
    question.update_validate()?;
    state.question.update_question(id, question).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn complete_question(
    State(state): State<AppState>,
    question_id: i32,
    form_id: i32,
) -> Result<impl IntoResponse, ResponseError> {
    state.question.complete_question(question_id, form_id).await?;
    Ok(StatusCode::NO_CONTENT)
}