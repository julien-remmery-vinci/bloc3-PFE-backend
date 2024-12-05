use axum::{http::StatusCode, response::IntoResponse, extract::{State, Path}, Json};
use crate::database::state::AppState;
use crate::models::form::Form;
use crate::services::forms::{create_form_in_db, read_form_in_db, update_form_in_db, delete_form_in_db};

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
) -> Result<Json<Form>, axum::response::Response> {
    let db_pool = &state.auth.db;
    let form = read_form_in_db(db_pool, form_id)
        .await
        .map_err(|e| {
            let error_message = format!("Failed to read form: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, error_message).into_response()
        })?;
    Ok(Json(form))
}

#[axum::debug_handler]
pub async fn update_form(
    State(state): State<AppState>,
    Path(form_id): Path<i32>,
    Json(updated_form): Json<Form>,
) -> Result<Json<Form>, axum::response::Response> {
    let db_pool = &state.auth.db;
    let form = update_form_in_db(db_pool, form_id, updated_form)
        .await
        .map_err(|e| {
            let error_message = format!("Failed to update form: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, error_message).into_response()
        })?;
    Ok(Json(form))
}

#[axum::debug_handler]
pub async fn delete_form(
    State(state): State<AppState>,
    Path(form_id): Path<i32>,
) -> Result<StatusCode, axum::response::Response> {
    let db_pool = &state.auth.db;
    delete_form_in_db(db_pool, form_id)
        .await
        .map_err(|e| {
            let error_message = format!("Failed to delete form: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, error_message).into_response()
        })?;
    Ok(StatusCode::NO_CONTENT)
}
