use crate::database::database::AppState;
use axum::extract::{Json, State};
use axum::http::StatusCode;
use serde_json::{json, Value as JsonValue};

pub async fn read_all(State(_state): State<AppState>) -> (StatusCode, axum::Json<JsonValue>) {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!("Not implemented"))
    )
}

pub async fn read_one(State(_state): State<AppState>) -> (StatusCode, axum::Json<JsonValue>) {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!("Not implemented"))
    )
}

pub async fn create(State(_state): State<AppState>) -> (StatusCode, axum::Json<JsonValue>) {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!("Not implemented"))
    )
}

pub async fn update(State(_state): State<AppState>) -> (StatusCode, axum::Json<JsonValue>) {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!("Not implemented"))
    )
}

pub async fn delete(State(_state): State<AppState>) -> (StatusCode, axum::Json<JsonValue>) {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!("Not implemented"))
    )
}