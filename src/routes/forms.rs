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
            CompleteForm, CreateForm, QuestionWithAnswers
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

// #[axum::debug_handler]
// pub async fn read_form(
//     State(state): State<AppState>,
//     Path(form_id): Path<i32>,
// ) -> Result<impl IntoResponse, FormError> {
//     let form = state.form.read_form_in_db(form_id)
//         .await
//         .map_err(|e| match e {
//             sqlx::Error::RowNotFound => FormError::NotFound,
//             _ => FormError::DbError(e),
//         })?;
//     Ok((StatusCode::OK, Json(form)))
// }

#[axum::debug_handler]
pub async fn read_forms_by_company(
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
        let templates = state.form.read_form_templates(form.form_id).await?;

        let mut new_form: CompleteForm = CompleteForm { 
            form_id: form.form_id, 
            company_id: form.company_id, 
            r#type: form.r#type.clone(), 
            templates,
            questions: Vec::new() 
        };

        let questions = state.question.read_all_by_form_id(form.form_id).await?;

        for question in &questions {
            let answers = state.answer.read_answers_by_question(question.question_id).await?;
            let user_answers = state.answer.read_answers_by_user_by_question(user.user_id, question.question_id, form.form_id).await?;

            new_form.questions.push(
                QuestionWithAnswers { 
                    question: question.clone(), 
                    answers, 
                    user_answers
                }
            );
        }

        forms_list.push(new_form);
    }

    Ok((StatusCode::OK, Json(forms_list)))
}
