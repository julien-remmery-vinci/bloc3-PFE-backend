use std::env;

use axum::extract::Request;
use axum::{extract::State, Json};

use crate::models::user::UserToken;
use crate::models::{
    credentials::Credentials,
    createuser::CreateUser,
    user::User
};
use crate::database::state::AppState;
use crate::errors::autherror::AuthError;
use crate::services::auth;

#[axum::debug_handler]
pub async fn login(
    State(state): State<AppState>,
    Json(credentials): Json<Credentials>,
) -> Result<Json<UserToken>, AuthError> {
    if credentials.invalid() {
        return Err(AuthError::BadRequest);
    }

    match state.auth.find_by_login(credentials.login.clone()).await? {
        None => return Err(AuthError::NoSuchUser),
        Some(user) => {
            if bcrypt::verify(&credentials.password, &user.password).map_err(AuthError::BCryptError)? {
                let token = auth::encode_jwt(credentials).map_err(AuthError::JWTError)?;
                return Ok(Json(UserToken { user, token }));
            } else {
                return Err(AuthError::WrongPassword);
            }
        }
    };
}

// TODO : Not return the password
pub async fn register(
    State(state): State<AppState>,
    Json(mut user): Json<CreateUser>,
) -> Result<Json<User>, AuthError> {
    if user.invalid() {
        return Err(AuthError::BadRequest);
    }

    match state.auth.find_by_login(user.login.clone()).await? {
        Some(_) => return Err(AuthError::Conflict),
        None => (),
    }

    match user.company_id {
        Some(company_id) => {
            let company = state.company.find_by_id(company_id).await
                .map_err(AuthError::DbError)?;
            if company.company_id == 0 {
                return Err(AuthError::NoSuchCompany);
            }
        }
        None => (),
    }

    let hashed_password = bcrypt::hash(&user.password, env::var("HASH_ROUNDS")
        .expect("HASH_ROUNDS must be set")
        .parse::<u32>()
        .unwrap())
        .map_err(AuthError::BCryptError)?;
    user.password = hashed_password;
    let created = state.auth.create_user(user).await?;
    Ok(Json(created))
}

pub async fn verify(
    request: Request
) -> Result<Json<User>, AuthError> {
    Ok(Json(request.extensions().get::<User>().unwrap().clone()))
}
