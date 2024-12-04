use axum::{extract::State, Json};

use crate::{
    models::credentials::Credentials,
    services::auth::OkResponse,
    database::state::AppState,
};

pub async fn login(
    State(state): State<AppState>,
    Json(user): Json<Credentials>,
) -> Result<Json<OkResponse>, crate::services::auth::AuthError> {
    let valid = state.auth.auth_query(user).await?;
    Ok(Json(valid))
}

// pub async fn register(
//     State(state): State<AppState>,
//     Json(user): Json<CreateUser>,
// ) -> Result<Json<OkResponse>, crate::services::auth::AuthError> {
//     let valid = state.auth.auth_query(user).await?;
//     Ok(Json(valid))
// }
