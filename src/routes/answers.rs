use axum::extract::Path;
use axum::Extension;
use axum::{extract::State, Json};
use serde::Deserialize;

use crate::errors::globalerror::GlobalError;
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
    pub answer: Option<String>,
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
    //si le contenue du champ answer de la table answers_esg est NULL alors answer de l'objet CreateAnswerUser est obligatoire
    let possible_answer = state.answer.read_possible_answer_by_id(answer_id).await?;
    if possible_answer.is_none() {
        if answer.answer.is_some() {
            return Err(AnswerError::BadRequest);
        }   
    }
    if possible_answer.is_some() {
        if answer.answer.is_none() {
            return Err(AnswerError::BadRequest);
        }
    }
    if answer.invalid() {
        return Err(AnswerError::BadRequest);
    }
    //check si l'id de l'answer exist
    match state.answer.read_answer_by_id(answer_id).await? {
        None => return Err(AnswerError::NoSuchAnswer),
        Some(_) => (),
    }
    //TODO check si le form existe
    //check si on a deja rep a cette answer
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
) -> Result<Json<Vec<Answer>>, GlobalError> {
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