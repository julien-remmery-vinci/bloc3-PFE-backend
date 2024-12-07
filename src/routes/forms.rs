use axum::{http::StatusCode, response::IntoResponse, extract::{State, Path}, Json};
use crate::database::state::AppState;
use crate::models::form::Form;
use crate::errors::form_error::FormError;

#[axum::debug_handler]
pub async fn create_form(
    State(state): State<AppState>,
    Json(new_form): Json<Form>,
) -> Result<impl IntoResponse, FormError> {
    if new_form.form_type.is_empty() {
        return Err(FormError::BadRequest);
    }

    match state.form.create_form_in_db(new_form).await {
        Ok(form) => Ok((StatusCode::CREATED, Json(form))),
        Err(error) => Err(FormError::DbError(error)),
    }
}

#[axum::debug_handler]
pub async fn read_form(
    State(state): State<AppState>,
    Path(form_id): Path<i32>,
) -> Result<impl IntoResponse, FormError> {
    let form = state.form.read_form_in_db(form_id)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => FormError::NotFound,
            _ => FormError::DbError(e),
        })?;
    Ok((StatusCode::OK, Json(form)))
}

#[axum::debug_handler]
pub async fn update_form(
    State(state): State<AppState>,
    Path(form_id): Path<i32>,
    Json(updated_form): Json<Form>,
) -> Result<impl IntoResponse, FormError> {
    if updated_form.form_type.is_empty() {
        return Err(FormError::BadRequest);
    }

    let form = state.form.update_form_in_db(form_id, updated_form)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => FormError::NotFound,
            _ => FormError::DbError(e),
        })?;
    Ok((StatusCode::OK, Json(form)))
}

#[axum::debug_handler]
pub async fn delete_form(
    State(state): State<AppState>,
    Path(form_id): Path<i32>,
) -> Result<impl IntoResponse, FormError> {
    state.form.delete_form_in_db(form_id)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => FormError::NotFound,
            _ => FormError::DbError(e),
        })?;
    Ok(StatusCode::NO_CONTENT)
}

#[axum::debug_handler]
pub async fn read_forms_by_user(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> Result<impl IntoResponse, FormError> {
    let forms = state.form.read_forms_by_user_in_db(user_id)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => FormError::NotFound,
            _ => FormError::DbError(e),
        })?;
    Ok((StatusCode::OK, Json(forms)))
}
