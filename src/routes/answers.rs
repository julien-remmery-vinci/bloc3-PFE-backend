use axum::extract::Path;
use axum::Extension;
use axum::{
    extract::State, 
    Json
};

use crate::errors::responserror::ResponseError;
use crate::models::answer::{AnswerUser, CreateAnswer, CreateAnswerUser};
use crate::{
    database::state::AppState,
    models::answer::Answer,
    models::user::User,
};

#[axum::debug_handler]
pub async fn create_answer(
    State(state): State<AppState>,
    Json(answer): Json<CreateAnswer>,
) -> Result<Json<Answer>, ResponseError> {
    if answer.invalid() {
        return Err(ResponseError::BadRequest(Some(String::from("Missing answer information"))));
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
) -> Result<Json<AnswerUser>, ResponseError> {
    //si le contenue du champ answer de la table answers_esg est NULL alors answer de l'objet CreateAnswerUser est obligatoire
    let possible_answer = state.answer.read_possible_answer_by_id(answer_id).await?;
    if possible_answer.is_none() {
        if answer.answer.is_some() {
            return Err(ResponseError::BadRequest(Some(String::from("Answer information not required"))));
        }   
    }
    if possible_answer.is_some() {
        if answer.answer.is_none() {
            return Err(ResponseError::BadRequest(Some(String::from("Missing answer information"))));
        }
    }
    if answer.invalid() {
        return Err(ResponseError::BadRequest(Some(String::from("Missing answer information"))));
    }
    //check si l'id de l'answer exist
    match state.answer.read_answer_by_id(answer_id).await? {
        None => return Err(ResponseError::NotFound(Some(String::from("Answer not found")))),
        Some(_) => (),
    }
    //check si le form existe
    match state.form.read_form_by_id(answer.form_id).await? {
        None => return Err(ResponseError::NotFound(Some(String::from("Form not found")))),
        Some(_) => (),
    }
    //check si on a deja rep a cette answer
    let user_id = user.user_id;
    match state.answer.read_answer_user_by_form_id(answer.form_id,user_id,answer_id).await? {
        Some(_) => return Err(ResponseError::Conflict(Some(String::from("Answer already exists")))),
        None => (),
    }
    //TODO check si il y un engagement forcé
    //TODO changé l'état de la question en answered
    // on ne peut pas avoir now et commitment_pact true en meme temps
    if answer.now && answer.commitment_pact {
        return Err(ResponseError::BadRequest(Some(String::from("You can't have now and commitment_pact true at the same time"))));
    }

    let valid = state.answer.create_answer_user(answer,user_id,answer_id).await?;
    Ok(Json(valid))
}

#[axum::debug_handler]
pub async fn read_answers_by_question(
    State(state): State<AppState>,
    Path(question_id): Path<i32>,
) -> Result<Json<Vec<Answer>>, ResponseError> {
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