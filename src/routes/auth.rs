pub mod auth {
    use axum::extract::{Json, State};
    use axum::http::StatusCode;
    use serde::Deserialize;
    use serde_json::{json, Value as JsonValue};

    use crate::models::user::User;

    #[derive(Deserialize)]
    pub struct UnsafeCredentials {
        login: String,
        password: String,
    }

    #[axum::debug_handler]
    pub async fn login(State(state): State<AppState>, Json(user): Json<UnsafeCredentials>) -> (StatusCode, axum::Json<JsonValue>) {
        let query = "SELECT login, password FROM pfe.users WHERE login = $1";
        match sqlx::query_as::<_, User>(&query)
        .bind(user.login)
        .fetch_all(&state.db).await {
            Ok(result) => {
                if result.is_empty() {
                    (
                        StatusCode::NOT_FOUND,
                        Json(json!("User not found"))
                    )
                } else if bcrypt::verify(&user.password, &result[0].password).unwrap() {
                    (
                        StatusCode::OK,
                        Json(json!({"login": result[0].login}))
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

    pub async fn register(State(state): State<AppState>, Json(user): Json<User>) -> (StatusCode, axum::Json<JsonValue>) {
        (
            StatusCode::NOT_IMPLEMENTED,
            Json(json!("Not implemented"))
        )
    }
}
