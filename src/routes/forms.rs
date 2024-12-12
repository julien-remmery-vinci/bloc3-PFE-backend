use axum::{
    extract::{Path, State}, http::StatusCode, response::IntoResponse, Extension, Json
};
use crate::{
    database::state::AppState, 
    errors::responserror::ResponseError, 
    models::{
        company::Company, form::{
            CompleteForm, CreateForm, Form, ModifiableForm, QuestionWithAnswers, QuestionWithUserAnswers
        }, question::Question, user::User
    }
};

#[derive(serde::Deserialize)]
pub struct SubmitValidation {
    confirmation: bool,
}

#[axum::debug_handler]
pub async fn create_form(
    State(state): State<AppState>,
    Json(new_form): Json<CreateForm>,
) -> Result<impl IntoResponse, ResponseError> {
    if new_form.invalid() {
        return Err(ResponseError::BadRequest(None));
    }

    let templates = state.company.read_company_templates(new_form.company_id).await?;

    let questions: Vec<Question> = state.question
        .read_all_used_questions(new_form.r#type.clone()).await?;

    match state.form.create_form_in_db(new_form, questions, templates).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(error) => Err(error),
    }
}

pub async fn read_forms_with_questions_and_answers(
    State(state): State<AppState>
) -> Result<impl IntoResponse, ResponseError> {
    let mut form = ModifiableForm {
        r#type: String::from("ESG"),
        questions: Vec::new(),
    };

    let questions = state.question.read_all_questions_by_type(form.r#type.clone()).await?;
    for question in questions {
        let answers = state.answer.read_answers_by_question(question.question_id).await?;
        form.questions.push(QuestionWithAnswers { question, answers });
    }
    Ok((StatusCode::OK, Json(form)))
}

#[axum::debug_handler]
pub async fn read_forms_by_user(
    State(state): State<AppState>,
    Extension(user): Extension<User>
) -> Result<impl IntoResponse, ResponseError> {
    if user.company_id.is_none() {
        return Err(ResponseError::BadRequest(None));
    }

    let company_id = user.company_id.unwrap();
    let forms_list: Vec<CompleteForm> = get_complete_forms(state, company_id).await?;

    Ok((StatusCode::OK, Json(forms_list)))
}

#[axum::debug_handler]
pub async fn submit_form(
    State(state): State<AppState>,
    Path(form_id): Path<i32>,
    Extension(user): Extension<User>,
    Json(confirmation): Json<SubmitValidation>
) -> Result<impl IntoResponse, ResponseError> {
    if form_id <= 0 {
        return Err(ResponseError::BadRequest(Some(String::from("Invalid form id"))));
    }

    let form = match state.form.read_form_by_id(form_id).await {
        Ok(form) => form.unwrap(),
        Err(_) => {
            return Err(ResponseError::NotFound(Some(String::from("Form not found"))))
        },
    };

    if form.company_id != user.company_id.unwrap() {
        return Err(ResponseError::Unauthorized(Some(String::from("You are not authorized to submit this form"))));
    }

    if !confirmation.confirmation {
        let pending_questions: Vec<i32> = state.form.get_pending_questions(form_id).await?;

        if pending_questions.len() > 0 {
            return Ok((StatusCode::BAD_REQUEST, Json(pending_questions)).into_response());
        }
    }

    state.form.user_submit_form(form_id).await?;
    Ok(StatusCode::OK.into_response())
}

#[axum::debug_handler]
pub async fn submit_validated_form(
    State(state): State<AppState>,
    Path(form_id): Path<i32>,
) -> Result<impl IntoResponse, ResponseError> {
    if form_id <= 0 {
        return Err(ResponseError::BadRequest(Some(String::from("Invalid form id"))));
    }

    match state.form.read_form_by_id(form_id).await {
        Ok(form) => form.unwrap(),
        Err(_) => {
            return Err(ResponseError::NotFound(Some(String::from("Form not found"))))
        },
    };

    state.form.submit_validated_form(form_id).await?;
    Ok((StatusCode::NO_CONTENT, Json("Form submitted")).into_response())
}

pub async fn get_complete_form(
    state: AppState,
    form: Form,
    company: Company,
) -> Result<CompleteForm, ResponseError> {
    let templates = state.form.read_form_templates(form.form_id).await?;

    let mut complete_form: CompleteForm = CompleteForm { 
        form_id: form.form_id, 
        company_id: form.company_id, 
        r#type: form.r#type.clone(),
        status: form.status.clone(),
        templates,
        questions: Vec::new() 
    };

    let mut questions = state.question.read_all_by_form_id(form.form_id).await?;
    questions.sort_by(|a, b| a.question_id.cmp(&b.question_id));
    for mut question in questions {
        question.question = question.question.clone().replace("XXX", &company.company_name);
        let answers = state.answer.read_answers_by_question(question.question_id).await?;
        let user_answers = state.answer.read_answers_by_user_by_question(question.question_id, form.form_id).await?;

        complete_form.questions.push(
            QuestionWithUserAnswers { 
                question: question.clone(), 
                answers, 
                user_answers
            }
        );
    }
    Ok(complete_form)
}

pub async fn get_complete_forms(
    state: AppState,
    company_id: i32,
) -> Result<Vec<CompleteForm>, ResponseError> {
    let forms = state.form.read_forms_by_company(company_id).await?;
    let company = match state.company.find_by_id(company_id).await {
        Ok(company) => company.unwrap(),
        Err(_) => return Err(ResponseError::NotFound(Some(String::from("Company not found")))),
    };

    let mut forms_list: Vec<CompleteForm> = Vec::new();
    for form in forms {
        let complete_form = get_complete_form(state.clone(), form.clone(), company.clone()).await?;
        forms_list.push(complete_form);
    }
    Ok(forms_list)
}