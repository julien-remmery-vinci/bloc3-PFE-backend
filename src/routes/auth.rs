use crate::database::database::AppState;
use axum::extract::{Json, State};
use axum::http::StatusCode;
use jsonwebtoken::{EncodingKey, Header};
use serde_json::{json, Value as JsonValue};
use chrono::{Utc, Duration};
use sqlx::encode;

use crate::models::unsafecredentials::{Claims, UnsafeCredentials};
use crate::models::user::User;
use crate::models::createuser::CreateUser;

pub async fn login(State(state): State<AppState>, Json(credentials): Json<UnsafeCredentials>) -> (StatusCode, axum::Json<JsonValue>) {
    if credentials.invalid() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!("Empty login or password"))
        )
    }

    let query: &str = "SELECT id, login, password FROM pfe.users WHERE login = $1";
    match sqlx::query_as::<_, User>(&query)
    .bind(credentials.login.clone())
    .fetch_one(&state.db).await {
        Ok(result) => {
            if result.login == "" {
                (
                    StatusCode::NOT_FOUND,
                    Json(json!("User not found"))
                )
            } else if bcrypt::verify(&credentials.password, &result.password).unwrap() {
                (
                    StatusCode::OK,
                    Json(json!({"token": create_token(credentials.clone()).unwrap().0}))
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

fn create_token(credentials: UnsafeCredentials) -> Result<Json<String>, StatusCode> {
    let claims = Claims {
    sub: credentials.login,
    exp: (chrono::Utc::now() + chrono::Duration::days(1)).timestamp() as usize,
    };

    let token = match jsonwebtoken::encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error Generating Token: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        },
    };

    Ok(Json(token))
}