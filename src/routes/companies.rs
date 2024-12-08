use axum::{
    extract::State, http::StatusCode, response::IntoResponse, Json
};

use crate::{
    database::state::AppState, 
    errors::responserror::ResponseError, 
    models::company::Company
};

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