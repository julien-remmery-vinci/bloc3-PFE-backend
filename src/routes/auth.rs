use axum::{extract::State, Json};

use crate::models::{
    credentials::Credentials,
    createuser::CreateUser,
    user::User,
    tokenresponse::TokenResponse,
};
use crate::database::state::AppState;
use crate::errors::autherror::AuthError;

#[axum::debug_handler]
pub async fn login(
    State(state): State<AppState>,
    Json(credentials): Json<Credentials>,
) -> Result<Json<TokenResponse>, AuthError> {
    let valid = state.auth.login_user(credentials).await?;
    Ok(Json(valid))
}

// TODO : Not return the password
pub async fn register(
    State(state): State<AppState>,
    Json(user): Json<CreateUser>,
) -> Result<Json<User>, AuthError> {
    let valid = state.auth.register_user(user).await?;
    Ok(Json(valid))
}
