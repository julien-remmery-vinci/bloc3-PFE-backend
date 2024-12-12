use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Extension;
use axum::{
    extract::State, 
    Json
};

use crate::errors::responserror::ResponseError;
use crate::models::answer::{AnswerUser, CreateAnswer, CreateAnswerUser, CreateAnswerValidation, UpdateScoreRequest, ValidatedAnswer};
use crate::{
    database::state::AppState,
    models::answer::Answer,
    models::user::User,
};
use crate::routes::questions::complete_question;

#[axum::debug_handler]
pub async fn create_answer(
    State(state): State<AppState>,
    Json(answer): Json<CreateAnswer>,
) -> Result<impl IntoResponse, ResponseError> {
    if answer.invalid() {
        return Err(ResponseError::BadRequest(Some(String::from("Missing answer information"))));
    }
    let valid = state.answer.create_answer(answer).await?;
    Ok((StatusCode::CREATED, Json(valid)))
}

#[axum::debug_handler]
pub async fn create_answer_for_user(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Path(answer_id): Path<i32>,
    Json(created_answer): Json<CreateAnswerUser>
) -> Result<impl IntoResponse, ResponseError> {
    if created_answer.invalid() {
        return Err(ResponseError::BadRequest(Some(String::from("Missing answer information"))));
    }
    
    //si le contenue du champ answer de la table answers_esg est NULL alors comment de l'objet CreateAnswerUser est obligatoire
    let answer: Answer = match state.answer.read_answer_by_id(answer_id).await? {
        Some(answer) => Some(answer).unwrap(),
        None => Err(ResponseError::NotFound(Some(String::from("Answer not found"))))?,
    };

    if answer.is_forced_comment && created_answer.comment.is_none() {
        return Err(ResponseError::BadRequest(Some(String::from("This answer has a forced comment"))));
    }

    if answer.answer.is_some() {
        if created_answer.now.is_none() {
            return Err(ResponseError::BadRequest(Some(String::from("Missing now field"))));
        }
        if created_answer.commitment_pact.is_none() {
            return Err(ResponseError::BadRequest(Some(String::from("Missing commitment pact field"))));
        }
    }
    
    // check si il y un engagement forcé
    if answer.is_forced_engagement && (created_answer.commitment_pact.is_none() || created_answer.commitment_pact.unwrap() == false) {
        return Err(ResponseError::BadRequest(Some(String::from("This answer has a forced engagement"))));
    }
    
    // on ne peut pas avoir now et commitment_pact true en meme temps
    if !answer.is_forced_engagement && answer.answer.is_some() && created_answer.now.unwrap() && created_answer.commitment_pact.unwrap() {
        return Err(ResponseError::BadRequest(Some(String::from("You can't have now and commitment_pact true at the same time"))));
    }
    
    //check si le form existe
    match state.form.read_form_by_id(created_answer.form_id).await? {
        None => return Err(ResponseError::NotFound(Some(String::from("Form not found")))),
        Some(_) => (),
    }
    
    //check si on a deja répondu a cette answer , si oui on supprime la reponse
    let user_id: i32 = user.user_id;
    match state.answer.read_answer_by_form_id(created_answer.form_id, answer_id).await? {
        Some(_) => {
            state.answer.delete_user_answer_by_form_id(created_answer.form_id, answer_id).await?;
        },
        None => (),
    }
    // Save the user's answer
    let valid: AnswerUser = state.answer.create_answer_user(created_answer.clone(), user_id, answer_id).await?;
    
    // Set the question status as COMPLETE
    complete_question(State(state), answer.question_id.unwrap(), created_answer.form_id).await?;

    Ok((StatusCode::CREATED, Json(valid)))
}

#[axum::debug_handler]
pub async fn read_answers_by_question(
    State(state): State<AppState>,
    Path(question_id): Path<i32>,
) -> Result<Json<Vec<Answer>>, ResponseError> {
    if question_id <= 0 {
        return Err(ResponseError::BadRequest(Some(String::from("Invalid question id"))));
    }
    let answers = state.answer.read_answers_by_question(question_id).await?;
    Ok(Json(answers))
}

pub async fn validate_user_answer(
    State(state): State<AppState>,
    Path(answer_id): Path<i32>,
    Extension(user): Extension<User>,
    Json(validated): Json<ValidatedAnswer>,
) -> Result<Json<AnswerUser>, ResponseError> {
    if answer_id <= 0 {
        return Err(ResponseError::BadRequest(Some(String::from("Invalid answer id"))));
    }

    let answer: Answer = match state.answer.read_answer_by_id(answer_id).await? {
        Some(answer) => Some(answer).unwrap(),
        None => Err(ResponseError::NotFound(Some(String::from("Answer not found"))))?,
    };

    match state.form.read_form_by_id(validated.form_id).await? {
        None => return Err(ResponseError::NotFound(Some(String::from("Form not found")))),
        Some(_) => (),
    }

    if answer.is_forced_engagement && validated.commitment_pact_verif.is_none() {
        return Err(ResponseError::BadRequest(Some(String::from("This answer has a forced engagement"))));
    }

    let user_answer: Option<AnswerUser> = state.answer
        .read_user_answer_by_question(
            answer_id, 
            answer.question_id.unwrap(), 
            validated.form_id)
        .await?;

    if user_answer.is_some() {
        let mut user_answer: AnswerUser = user_answer.unwrap();
        user_answer.comment = validated.comment.clone();
        user_answer.now_verif = validated.now_verif.clone();
        user_answer.commitment_pact_verif = validated.commitment_pact_verif.clone();
        let validated = state.answer.validate_user_answer(user_answer).await?;
        return Ok(Json(validated))
    }

    let validation: CreateAnswerValidation = CreateAnswerValidation {
        answer_id,
        form_id: validated.form_id,
        user_id: user.user_id,
        comment: validated.comment.clone(),
        now_verif: validated.now_verif.clone(),
        commitment_pact_verif: validated.commitment_pact_verif.clone(),
    };

    let validated: AnswerUser = state.answer.insert_answer_validation(validation).await?;
    Ok(Json(validated))
}

#[axum::debug_handler]
pub async fn update_answer_score(
    State(state): State<AppState>,
    Path(answer_id): Path<i32>,
    Json(scores): Json<UpdateScoreRequest>,
) -> Result<Json<Answer>, ResponseError> {
    if answer_id <= 0 {
        return Err(ResponseError::BadRequest(Some(String::from("Invalid answer id"))));
    }

    if scores.invalid() {
        return Err(ResponseError::BadRequest(Some(String::from("Invalid scores: scores cannot be negative"))));
    }

    let answer = match state.answer.read_answer_by_id(answer_id).await? {
        Some(answer) => answer,
        None => return Err(ResponseError::NotFound(Some(String::from("Answer not found")))),
    };

    let score_now = scores.score_now.unwrap_or(answer.score_now);
    let score_commitment_pact = scores.score_commitment_pact.unwrap_or(answer.score_commitment_pact);

    let updated = state.answer.update_answer_score(answer_id, score_now, score_commitment_pact).await?;
    Ok(Json(updated))
}