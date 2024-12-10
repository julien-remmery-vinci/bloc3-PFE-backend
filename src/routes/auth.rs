use axum::extract::Request;
use axum::{
    extract::State, 
    Json
};
use crate::errors::responserror::ResponseError;
use crate::models::user::UserToken;
use crate::models::{
    credentials::Credentials,
    user::CreateUser,
    user::User
};
use crate::database::state::AppState;
use crate::services::auth::{self, hash_password};

#[axum::debug_handler]
pub async fn login(
    State(state): State<AppState>,
    Json(credentials): Json<Credentials>,
) -> Result<Json<UserToken>, ResponseError> {
    if credentials.invalid() {
        return Err(ResponseError::BadRequest(Some(String::from("Invalid credentials"))));
    }

    match state.auth.find_by_login(credentials.login.clone()).await? {
        None => return Err(ResponseError::NotFound(Some(String::from("User not found")))),
        Some(user) => {
            if bcrypt::verify(&credentials.password, &user.password).map_err(ResponseError::BCryptError)? {
                let token = auth::encode_jwt(credentials)?;
                return Ok(Json(UserToken { user, token }));
            } else {
                return Err(ResponseError::Unauthorized(Some(String::from("Wrong password"))));
            }
        }
    };
}

// TODO : Not return the password
pub async fn register(
    State(state): State<AppState>,
    Json(mut user): Json<CreateUser>,
) -> Result<Json<User>, ResponseError> {
    if user.invalid() {
        return Err(ResponseError::BadRequest(Some(String::from("Missing user information"))));
    }

    match state.auth.find_by_login(user.login.clone()).await? {
        Some(_) => return Err(ResponseError::Conflict(Some("User already exists".to_string()))),
        None => (),
    }

    match user.company_id {
        Some(company_id) => {
            match state.company.find_by_id(company_id).await? {
                    Some(_) => (),
                    None => return Err(ResponseError::NotFound(Some("Company not found".to_string()))),
                }
        }
        None => (),
    }

    let hashed_password = match hash_password(user.password.clone()) {
        Ok(hashed_password) => hashed_password,
        Err(error) => return Err(error),
    };
    user.password = hashed_password;
    let created = state.auth.create_user(user).await?;
    Ok(Json(created))
}

pub async fn verify(
    request: Request
) -> Result<Json<User>, ResponseError> {
    Ok(Json(request.extensions().get::<User>().unwrap().clone()))
}
