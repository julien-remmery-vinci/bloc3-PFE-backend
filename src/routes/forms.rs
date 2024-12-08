use axum::{
    extract::State, 
    http::StatusCode, 
    response::IntoResponse, 
    Extension, 
    Json
};
use crate::{
    database::state::AppState, 
    errors::responserror::ResponseError, 
    models::{
        form::{
            CompleteForm, CreateForm, Form, QuestionWithAnswers
        }, 
        question::Question, 
        user::User
    }
};

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
        Err(error) => Err(ResponseError::DbError(error)),
    }
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
    let forms = state.form.read_forms_by_company(company_id)
        .await?;

    let mut forms_list: Vec<CompleteForm> = Vec::new();
    for form in forms {
        let complete_form = get_complete_form(state.clone(), form.clone()).await?;
        forms_list.push(complete_form);
    }

    Ok((StatusCode::OK, Json(forms_list)))
}

pub async fn get_complete_form(
    state: AppState,
    form: Form,
) -> Result<CompleteForm, ResponseError> {
    let templates = state.form.read_form_templates(form.form_id).await?;

    let mut complete_form: CompleteForm = CompleteForm { 
        form_id: form.form_id, 
        company_id: form.company_id, 
        r#type: form.r#type.clone(), 
        templates,
        questions: Vec::new() 
    };

    let questions = state.question.read_all_by_form_id(form.form_id).await?;

    for question in &questions {
        let answers = state.answer.read_answers_by_question(question.question_id).await?;
        let user_answers = state.answer.read_answers_by_user_by_question(question.question_id, form.form_id).await?;

        complete_form.questions.push(
            QuestionWithAnswers { 
                question: question.clone(), 
                answers, 
                user_answers
            }
        );
    }
    Ok(complete_form)
}