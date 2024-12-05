use axum::{http::StatusCode, response::IntoResponse, extract::{State, Path}, Json};
use crate::database::state::AppState;
use crate::models::form::Form;
use crate::services::forms::{create_form_in_db, read_form_in_db, update_form_in_db, delete_form_in_db};
use crate::errors::form_error::FormError;

#[axum::debug_handler]
pub async fn create_form(
    State(state): State<AppState>,
    Json(new_form): Json<Form>,
) -> Result<Json<Form>, axum::response::Response> {
    let db_pool = &state.auth.db;
    let form = create_form_in_db(db_pool, new_form)
        .await
        .map_err(|e| {
            let error_message = format!("Failed to create form: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, error_message).into_response()
        })?;
    Ok(Json(form))
}

#[axum::debug_handler]
pub async fn read_form(
    State(state): State<AppState>,
    Path(form_id): Path<i32>,
) -> Result<Json<Form>, FormError> {
    let db_pool = &state.auth.db;
    let form = read_form_in_db(db_pool, form_id)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => FormError::NotFound,
            _ => FormError::DbError(e),
        })?;
    Ok(Json(form))
}

#[axum::debug_handler]
pub async fn update_form(
    State(state): State<AppState>,
    Path(form_id): Path<i32>,
    Json(updated_form): Json<Form>,
) -> Result<Json<Form>, FormError> {
    // Validation
    if updated_form.form_type.is_empty() || updated_form.template.is_empty() {
        return Err(FormError::BadRequest);
    }

    let db_pool = &state.auth.db;
    let form = update_form_in_db(db_pool, form_id, updated_form)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => FormError::NotFound,
            _ => FormError::DbError(e),
        })?;
    Ok(Json(form))
}

#[axum::debug_handler]
pub async fn delete_form(
    State(state): State<AppState>,
    Path(form_id): Path<i32>,
) -> Result<StatusCode, FormError> {
    let db_pool = &state.auth.db;
    delete_form_in_db(db_pool, form_id)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => FormError::NotFound,
            _ => FormError::DbError(e),
        })?;
    Ok(StatusCode::NO_CONTENT)
}
