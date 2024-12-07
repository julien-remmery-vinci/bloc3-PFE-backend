use axum::extract::Path;
use axum::Extension;
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
}

#[derive(Deserialize)]
pub struct CreateAnswerUser {
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
    Extension(user): Extension<User>,
    Path(answer_id): Path<i32>,
    Json(answer): Json<CreateAnswerUser>
) -> Result<Json<AnswerUser>, AnswerError> {
    if answer.invalid() {
        return Err(AnswerError::BadRequest);
    }
    match state.answer.read_answer_by_id(answer_id).await? {
        None => return Err(AnswerError::NoSuchAnswer),
        Some(_) => (),
    }
    //TODO check si le form existe
    let user_id = user.user_id;
    match state.answer.read_answer_user_by_form_id(answer.form_id,user_id,answer_id).await? {
        Some(_) => return Err(AnswerError::Conflict),
        None => (),
    }

    let valid = state.answer.create_answer_user(answer,user_id,answer_id).await?;
    Ok(Json(valid))
}

#[axum::debug_handler]
pub async fn read_answers_by_question(
    State(state): State<AppState>,
    Path(question_id): Path<i32>,
) -> Result<Json<Vec<Answer>>, AnswerError> {
    let answers = state.answer.read_answers_by_question(question_id).await?;
    Ok(Json(answers))
}

impl CreateAnswerUser {
    pub fn invalid(&self) -> bool {
        self.form_id == 0
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