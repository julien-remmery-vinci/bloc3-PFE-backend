use std::env;

use axum::{body::Body, extract::{Request, State}, http::{self, StatusCode}, middleware::Next, response::Response};
use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};

use crate::{
    database::state::AppState, 
    errors::autherror::AuthError, 
    models::{claims::Claims, user::User}
};

fn decode_jwt(jwt_token: String) -> Result<TokenData<Claims>, StatusCode> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let result: Result<TokenData<Claims>, StatusCode> = decode(
        &jwt_token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
    result
}

/**
 * Middleware to check if the user is authorized to access the route
 * Some of the code associated to this middleware is inspired from :
 *  - https://blog.logrocket.com/using-rust-axum-build-jwt-authentication-api/
 */
async fn authorize_request(State(state): State<AppState>, req: &mut Request) -> Result<User, AuthError> {
    let auth_header = req.headers_mut().get(http::header::AUTHORIZATION);
    let auth_header = match auth_header {
        Some(header) => header.to_str().map_err(|_| AuthError::EmptyHeaderError)?,
        None => return Err(AuthError::NoTokenError),
    };

    let mut header = auth_header.split_whitespace();
    let _bearer = header.next();
    let token = header.next();
    let token = match token {
        Some(t) => t,
        None => return Err(AuthError::NoTokenError),
    };

    let token_data = match decode_jwt(token.to_string()) {
        Ok(data) => data,
        Err(_) => return Err(AuthError::TokenDecodeError),
    };

    let current_user = state.auth.find_by_login(token_data.claims.sub).await?;
    if current_user.id == 0 {
        return Err(AuthError::Unauthorized);
    }
    Ok(current_user)
}

pub async fn authorize_user(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response<Body>, AuthError> {
    let user = authorize_request(State(state), &mut req).await?;
    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}

pub async fn authorize_admin(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response<Body>, AuthError> {
    let user = authorize_request(State(state), &mut req).await?;
    if user.is_admin() {
        req.extensions_mut().insert(user);
        Ok(next.run(req).await)
    } else {
        Err(AuthError::Forbidden)
    }
}