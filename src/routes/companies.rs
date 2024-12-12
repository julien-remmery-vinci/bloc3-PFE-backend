use axum::{
    extract::{Path, State}, http::StatusCode, response::IntoResponse, Extension, Json
};

use crate::{
    database::state::AppState, 
    errors::responserror::ResponseError, 
    models::{company::{Company, CompanyWithCompleteForms}, form::CompleteForm, user::User},
    routes::forms::get_complete_forms,
};

#[axum::debug_handler]
pub async fn read_one_company(
    State(state): State<AppState>,
    Path(company_id): Path<i32>,
) -> Result<Json<CompanyWithCompleteForms>, ResponseError> {
    if company_id <= 0 {
        return Err(ResponseError::BadRequest(Some(String::from("Invalid company id"))));
    }

    let company = match state.company.find_by_id(company_id).await? {
        Some(company) => company,
        None => return Err(ResponseError::NotFound(Some(String::from("Company not found")))),
    };

    let forms_list: Vec<CompleteForm> = get_complete_forms(state, company_id).await?;

    Ok(Json(CompanyWithCompleteForms { company, forms: forms_list }))
}

#[axum::debug_handler]
pub async fn read_all_companies(
    State(state): State<AppState>,
) -> Result<Json<Vec<Company>>, ResponseError> {
    let companies: Vec<Company> = state.company.get_companies().await?;
    Ok(Json(companies))
}

pub async fn create_company(
    State(state): State<AppState>,
    Json(company): Json<Company>,
) -> Result<impl IntoResponse, ResponseError> {
    if company.invalid() {
        return Err(ResponseError::BadRequest(Some(String::from("Invalid company data"))));
    }
    match state.company.read_by_company_number(company.company_number.clone()).await? {
        Some(_) => return Err(ResponseError::Conflict(Some(String::from("Company already exists")))),
        None => (),
    }

    state.company.create_company(company).await?;
    Ok(StatusCode::CREATED)
}

pub async fn company_forms_status(
    State(state): State<AppState>,
    Path(company_id): Path<i32>,
    Extension(user): Extension<User>
) -> Result<impl IntoResponse, ResponseError> {
    if company_id <= 0 {
        return Err(ResponseError::BadRequest(Some(String::from("Invalid company id"))));
    }

    if user.role != "admin" && user.company_id.unwrap() != company_id {
        return Err(ResponseError::Unauthorized(Some(String::from("You are not authorized to view this company's forms status"))));
    }

    match state.company.find_by_id(company_id).await? {
        Some(_) => (),
        None => return Err(ResponseError::NotFound(Some(String::from("Company not found")))),
    };

    let forms_list: Vec<CompleteForm> = get_complete_forms(state.clone(), company_id).await?;

    let last_form = match forms_list.last() {
        Some(form) => form,
        None => return Err(ResponseError::BadRequest(Some(String::from("No forms found for specified company")))),
    };
    let mut total_answers = 0;
    let mut total_user_answers = 0;

    // for question in last_form.questions.iter() {
    //     total_answers += question.answers.len();
    //     total_user_answers += question.user_answers.len();
    // }

    let questions_form = state.question.read_all_questions_forms_by_form_id(last_form.form_id).await?;
    for question in questions_form {
        total_answers +=1 ;
        if question.question_status == "COMPLETE" {
            total_user_answers += 1;
        }
    }

    Ok(Json((total_user_answers as f64 / total_answers as f64) * 100 as f64))
}

pub async fn get_user_company(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
) -> Result<Json<Company>, ResponseError> {
    if user.company_id.is_none() || user.company_id.unwrap() <= 0 {
        return Err(ResponseError::BadRequest(Some(String::from("Invalid user company id"))));
    }

    match state.company.find_by_id(user.company_id.unwrap()).await? {
        Some(company) => Ok(Json(company)),
        None => Err(ResponseError::NotFound(Some(String::from("Company not found")))),
    }
}