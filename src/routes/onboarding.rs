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

    match state.company.read_by_company_number(data.company_number.clone()).await? {
        Some(_) => return Err(ResponseError::Conflict(Some(String::from("Company number already used")))),
        None => (),
    }

    match state.onboarding.read_by_company_number(data.company_number.clone()).await? {
        Some(_) => return Err(ResponseError::Conflict(Some(String::from("Company number already used")))),
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
    match state.onboarding.read_by_id(onboarding_id).await? {
        Some(onboarding) => {
            if onboarding.accepted() || onboarding.rejected() {
                return Err(ResponseError::BadRequest(Some(String::from("Onboarding already accepted"))));
            }
        },
        None => return Err(ResponseError::NotFound(Some(String::from("Onboarding not found")))),
    }

    let onboarding: Onboarding = match state.onboarding.accept_onboarding(onboarding_id).await {
        Ok(onboarding) => onboarding.unwrap(),
        Err(e) => return Err(e),
    };

    let company: Company = Company {
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
    let company: Company = state.company.create_company(company).await?;

    if onboarding.is_owner && onboarding.nb_workers > 0 && onboarding.sells_products {
        let template: String = String::from("ALL");
        state.company.insert_company_templates(company.company_id.unwrap(), template).await?;
    } else {
        if onboarding.sells_products {
            let template: String = String::from("PRODUITS");
            state.company.insert_company_templates(company.company_id.unwrap(), template).await?;
        } else {
            let template: String = String::from("FACILITY");
            state.company.insert_company_templates(company.company_id.unwrap(), template).await?;
        }
        if onboarding.nb_workers > 0 {
            let template: String = String::from("WORKERS");
            state.company.insert_company_templates(company.company_id.unwrap(), template).await?;
        }
        if onboarding.is_owner{
            let template: String = String::from("OWNED FACILITY");
            state.company.insert_company_templates(company.company_id.unwrap(), template).await?;
        }
    }

    let user: CreateUser = CreateUser {
        company_id: company.company_id,
        login: onboarding.email.clone(),
        password: onboarding.password.clone(),
        firstname: onboarding.firstname.clone(),
        lastname: onboarding.lastname.clone(),
        role: "user".to_string(),
    };
    state.auth.create_user(user).await?;
    let new_form: CreateForm = CreateForm {
        company_id: company.company_id.unwrap(),
        r#type: String::from("ESG"),
    };
    create_form(State(state), Json(new_form)).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn reject_onboarding(
    State(state): State<AppState>,
    Path(onboarding_id): Path<i32>,
) -> Result<impl IntoResponse, ResponseError> {
    if onboarding_id < 1 {
        return Err(ResponseError::BadRequest(Some(String::from("Invalid onboarding ID"))));
    }
    match state.onboarding.read_by_id(onboarding_id).await? {
        Some(onboarding) => {
            if onboarding.rejected() {
                return Err(ResponseError::BadRequest(Some(String::from("Onboarding already rejected"))));
            }
        },
        None => return Err(ResponseError::NotFound(Some(String::from("Onboarding not found")))),
    }

    state.onboarding.reject_onboarding(onboarding_id).await?;

    Ok(StatusCode::NO_CONTENT)
}