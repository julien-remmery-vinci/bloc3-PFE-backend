use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};

use crate::{database::state::AppState, errors::responserror::ResponseError, models::{company::Company, form::CreateForm, onboarding::Onboarding, user::CreateUser}, services::auth::hash_password};

use super::forms::create_form;

pub async fn submit_onboarding(
    State(state): State<AppState>,
    Json(mut data): Json<Onboarding>,
) -> Result<Json<Onboarding>, ResponseError> {
    if data.invalid() {
        return Err(ResponseError::BadRequest(Some(String::from("Invalid onboarding data"))));
    }

    match state.onboarding.read_request_by_email(data.email.clone()).await? {
        Some(_) => return Err(ResponseError::Conflict(Some(String::from("Email already used")))),
        None => (),
    }

    let hashed_password = match hash_password(data.password.clone()) {
        Ok(password) => password,
        Err(error) => return Err(error),
    };

    data.password = hashed_password;

    state.onboarding.submit_onboarding(data).await.map(Json)
}

pub async fn read_all_onboarding(
    State(state): State<AppState>,
) -> Result<Json<Vec<Onboarding>>, ResponseError> {
    state.onboarding.read_all_onboarding().await.map(Json)
}

pub async fn read_all_pending_onboarding(
    State(state): State<AppState>,
) -> Result<Json<Vec<Onboarding>>, ResponseError> {
    state.onboarding.read_all_pending_onboarding().await.map(Json)
}

pub async fn read_all_rejected_onboarding(
    State(state): State<AppState>,
) -> Result<Json<Vec<Onboarding>>, ResponseError> {
    state.onboarding.read_all_rejected_onboarding().await.map(Json)
}

pub async fn accept_onboarding(
    State(state): State<AppState>,
    Path(onboarding_id): Path<i32>,
) -> Result<impl IntoResponse, ResponseError> {
    if onboarding_id < 1 {
        return Err(ResponseError::BadRequest(Some(String::from("Invalid onboarding ID"))));
    }
    let onboarding = state.onboarding.accept_onboarding(onboarding_id).await?;
    if onboarding.is_none() {
        return Err(ResponseError::NotFound(Some(String::from("Onboarding not found"))));
    }
    let onboarding = onboarding.unwrap();

    let company = Company {
        company_id: None,
        company_name: onboarding.company_name.clone(),
        company_number: onboarding.company_number.clone(),
        legal_form: onboarding.legal_form.clone(),
        office_address: onboarding.office_address.clone(),
        website: Some(onboarding.website.clone()),
        nace_code: onboarding.nace_code.clone(),
        revenue: Some(onboarding.revenue.clone()),
        nb_workers: Some(onboarding.nb_workers.clone()),
        dispute: onboarding.dispute.clone()
    };
    let company = state.company.create_company(company).await?;
    let user = CreateUser {
        company_id: company.company_id,
        login: onboarding.email.clone(),
        password: onboarding.password.clone(),
        firstname: onboarding.firstname.clone(),
        lastname: onboarding.lastname.clone(),
        role: "user".to_string(),
    };
    state.auth.create_user(user).await?;
    let new_form = CreateForm {
        company_id: company.company_id.unwrap(),
        r#type: String::from("ESG"),
    };
    create_form(State(state), Json(new_form)).await?;
    Ok(StatusCode::NO_CONTENT)
}