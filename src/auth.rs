pub mod auth {
    use axum::extract::{Path, Query, Json, State};
    use axum::http::StatusCode;
    use serde::Deserialize;
    use serde_json::{json, Value as JsonValue};
    use crate::database::database::AppState;
    use crate::users::users::User;

    #[derive(Deserialize)]
    pub struct CreateUser {
        login: String,
        password: String,
    }

    #[axum::debug_handler]
    pub async fn login(State(state): State<AppState>, Json(user): Json<CreateUser>) -> (StatusCode, axum::Json<JsonValue>) {
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
}
