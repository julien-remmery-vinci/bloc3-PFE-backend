use axum::{extract::State, Json};

use crate::models::{
    credentials::Credentials,
    createuser::CreateUser,
    user::User,
    tokenresponse::TokenResponse,
};
use crate::database::state::AppState;
use crate::errors::autherror::AuthError;
use crate::services::auth;

#[axum::debug_handler]
pub async fn login(
    State(state): State<AppState>,
    Json(credentials): Json<Credentials>,
) -> Result<Json<TokenResponse>, AuthError> {
    if credentials.invalid() {
        return Err(AuthError::BadRequest);
    }

    let user = state.auth.find_by_login(credentials.login.clone()).await
        .map_err(AuthError::DbError)?;

    if user.id == 0 {
        return Err(AuthError::NoSuchUser);
    }
    else if bcrypt::verify(&credentials.password, &user.password).map_err(AuthError::BCryptError)? {
        let token = auth::encode_jwt(credentials).map_err(AuthError::JWTError)?;
        return Ok(Json(TokenResponse { token }));
    } else {
        return Err(AuthError::WrongPassword);
    }
}

// TODO : Not return the password
pub async fn register(
    State(state): State<AppState>,
    Json(mut user): Json<CreateUser>,
) -> Result<Json<User>, AuthError> {
    if user.invalid() {
        return Err(AuthError::BadRequest);
    }

    let found_user = state.auth.find_by_login(user.login.clone()).await
        .map_err(AuthError::DbError)?;
    if found_user.id != 0 {
        return Err(AuthError::Conflict);
    }
    let hashed_password = bcrypt::hash(&user.password, 12).map_err(AuthError::BCryptError)?;
    user.password = hashed_password;
    let created = state.auth.create_user(user).await.map_err(AuthError::DbError)?;
    Ok(Json(created))
}
