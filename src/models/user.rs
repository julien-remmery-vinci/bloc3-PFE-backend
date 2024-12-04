use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Clone, serde::Serialize)]
pub struct User {
    pub id: i32,
    pub login: String,
    pub password: String,
}