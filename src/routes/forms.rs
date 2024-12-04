use axum::{http::StatusCode, response::IntoResponse, extract::State, Json};
use crate::database::state::AppState;
use crate::models::form::Form;
use crate::services::forms::create_form_in_db;

#[axum::debug_handler]
pub async fn create(
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
