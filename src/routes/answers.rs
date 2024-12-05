use axum::extract::Request;
use axum::{extract::State, Json};
use serde::Deserialize;

use crate::models::answerusers::AnswerUser;
use crate::{
    database::state::AppState,
    errors::answer_error::AnswerError,
    models::answers::Answer,
    models::user::User,
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

#[derive(Deserialize)]
pub struct CreateAnswerUser {
    pub answer_id: i32,
    pub form_id: i32,
    pub now: bool,
    pub commitment_pact: bool,
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

#[axum::debug_handler]
pub async fn create_answer_for_user(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Json(answer): Json<CreateAnswerUser>,
) -> Result<Json<AnswerUser>, AnswerError> {
    if answer.invalid() {
        return Err(AnswerError::BadRequest);
    }
    let user_id = headers.get("user-id").unwrap().to_str().unwrap().parse::<i32>().unwrap();
    let valid = state.answer.create_answer_user(answer,user_id).await?;
    Ok(Json(valid))
}

impl CreateAnswerUser {
    pub fn invalid(&self) -> bool {
        self.answer_id == 0
            || self.form_id == 0
    }
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