use axum::{extract::State, Json};

use crate::{database::state::AppState, errors::company_error::CompanyError, models::company::Company};

#[axum::debug_handler]
pub async fn get_company(
    State(state): State<AppState>,
) -> Result<Json<Vec<Company>>, CompanyError> {
    let companies = state.company.get_companies().await?;
    Ok(Json(companies))
}