use axum::{
    extract::{Path, State}, http::StatusCode, response::IntoResponse, Json
};

use crate::{
    database::state::AppState, 
    errors::responserror::ResponseError, 
    models::{company::{self, Company, CompanyWithCompleteForms}, form::CompleteForm},
    routes::forms::get_complete_form
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

    let forms = state.form.read_forms_by_company(company_id).await?;

    let mut forms_list: Vec<CompleteForm> = Vec::new();
    for form in forms {
        let complete_form = get_complete_form(state.clone(), form.clone()).await?;
        forms_list.push(complete_form);
    }

    Ok(Json(CompanyWithCompleteForms { company, forms: forms_list }))
}

#[axum::debug_handler]
pub async fn read_all_companies(
    State(state): State<AppState>,
) -> Result<Json<Vec<Company>>, ResponseError> {
    let companies = state.company.get_companies().await?;
    Ok(Json(companies))
}

pub async fn create_company(
    State(state): State<AppState>,
    Json(company): Json<Company>,
) -> Result<impl IntoResponse, ResponseError> {
    match state.company.find_by_company_number(company.company_number.clone()).await? {
        Some(_) => return Err(ResponseError::Conflict(Some(String::from("Company already exists")))),
        None => (),
    }

    state.company.create_company(company).await?;
    Ok(StatusCode::CREATED)
}