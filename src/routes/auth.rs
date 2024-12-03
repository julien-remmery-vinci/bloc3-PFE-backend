use crate::database::database::AppState;
use axum::extract::{Json, State};
use axum::http::StatusCode;
use serde_json::{json, Value as JsonValue};

use crate::models::unsafecredentials::UnsafeCredentials;
use crate::models::user::User;
use crate::models::createuser::CreateUser;

pub async fn login(State(state): State<AppState>, Json(user): Json<UnsafeCredentials>) -> (StatusCode, axum::Json<JsonValue>) {
    if user.login == "" || user.password == "" {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!("Empty login or password"))
        )
    }

    let query: &str = "SELECT id, login, password FROM pfe.users WHERE login = $1";
    match sqlx::query_as::<sqlx::Postgres, User>(&query)
    .bind(user.login)
    .fetch_one(&state.db).await {
        Ok(result) => {
            if result.login == "" {
                (
                    StatusCode::NOT_FOUND,
                    Json(json!("User not found"))
                )
            } else if bcrypt::verify(&user.password, &result.password).unwrap() {
                (
                    StatusCode::OK,
                    Json(json!({"login": result.login}))
                )
            } else {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(json!("Wrong password"))
                )
            }
        },
        Err(error) => {
            println!("Error: {:?}", error);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!("Internal Server Error"))
            )
        }
    }
}

pub async fn register(State(state): State<AppState>, Json(user): Json<CreateUser>) -> (StatusCode, axum::Json<JsonValue>) {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!("Not implemented"))
    )
}