use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Extension, Json};
use crate::{database::state::AppState, errors::globalerror::GlobalError, models::{answers::Answer, company, form::{self, CreateForm, FormWithQuestions, QuestionWithAnswers}, question::Question, user::User}};
use crate::models::form::Form;
use crate::errors::form_error::FormError;

#[axum::debug_handler]
pub async fn create_form(
    State(state): State<AppState>,
    Json(new_form): Json<CreateForm>,
) -> Result<impl IntoResponse, GlobalError> {
    if new_form.invalid() {
        return Err(GlobalError::BadRequest);
    }

    let questions: Vec<Question> = state.question
        .read_all_used_questions(new_form.r#type.clone()).await?;

    match state.form.create_form_in_db(new_form, questions).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(error) => Err(GlobalError::DbError(error)),
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

// #[axum::debug_handler]
// pub async fn update_form(
//     State(state): State<AppState>,
//     Path(form_id): Path<i32>,
//     Json(updated_form): Json<Form>,
// ) -> Result<impl IntoResponse, FormError> {
//     if updated_form.form_type.is_empty() {
//         return Err(FormError::BadRequest);
//     }

//     let form = state.form.update_form_in_db(form_id, updated_form)
//         .await
//         .map_err(|e| match e {
//             sqlx::Error::RowNotFound => FormError::NotFound,
//             _ => FormError::DbError(e),
//         })?;
//     Ok((StatusCode::OK, Json(form)))
// }

// #[axum::debug_handler]
// pub async fn delete_form(
//     State(state): State<AppState>,
//     Path(form_id): Path<i32>,
// ) -> Result<impl IntoResponse, FormError> {
//     state.form.delete_form_in_db(form_id)
//         .await
//         .map_err(|e| match e {
//             sqlx::Error::RowNotFound => FormError::NotFound,
//             _ => FormError::DbError(e),
//         })?;
//     Ok(StatusCode::NO_CONTENT)
// }

#[axum::debug_handler]
pub async fn read_forms_by_company(
    State(state): State<AppState>,
    Extension(user): Extension<User>
) -> Result<impl IntoResponse, GlobalError> {
    if user.company_id.is_none() {
        return Err(GlobalError::BadRequest);
    }

    let company_id = user.company_id.unwrap();
    let forms = state.form.read_forms_by_company(company_id)
        .await?;

    let mut forms_list: Vec<FormWithQuestions> = Vec::new();
    for form in forms {
        let questions = state.question.read_all_by_form_id(form.form_id).await?;

        // let mut questions_with_answers = Vec::new();
        // for question in questions {
        //     let answers = state.answer.read_answers_by_question(question.question_id)
        //         .await?;

        //     questions_with_answers.push(QuestionWithAnswers { question, answers });
        // }

        forms_list.push(FormWithQuestions { form, questions });
    }

    Ok((StatusCode::OK, Json(forms_list)))
}
