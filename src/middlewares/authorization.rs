use std::env;

use axum::{body::Body, extract::{Request, State}, http::{self, StatusCode}, middleware::Next, response::Response};
use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};

use crate::{
    database::state::AppState, errors::responserror::ResponseError, models::{claims::Claims, user::User}
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
async fn authorize_request(State(state): State<AppState>, req: &mut Request) -> Result<User, ResponseError> {
    let auth_header = req.headers_mut().get(http::header::AUTHORIZATION);
    let auth_header = match auth_header {
        Some(header) => header.to_str().map_err(|_| ResponseError::BadRequest(Some(String::from("Empty header is not allowed"))))?,
        None => return Err(ResponseError::Unauthorized(Some(String::from("Please add the JWT token to the header")))),
    };

    let mut header = auth_header.split_whitespace();
    let _bearer = header.next();
    let token = header.next();
    let token = match token {
        Some(t) => t,
        None => return Err(ResponseError::Unauthorized(Some(String::from("Please add the JWT token to the header")))),
    };

    let token_data = match decode_jwt(token.to_string()) {
        Ok(data) => data,
        Err(_) => return Err(ResponseError::Unauthorized(Some(String::from("Unable to decode token")))),
    };
    match state.auth.find_by_login(token_data.claims.sub).await? {
        Some(user) => Ok(user),
        None => Err(ResponseError::Unauthorized(None)),
    }
}

pub async fn authorize_user(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response<Body>, ResponseError> {
    let user = authorize_request(State(state), &mut req).await?;
    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}

pub async fn authorize_admin(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response<Body>, ResponseError> {
    let user = authorize_request(State(state), &mut req).await?;
    if user.is_admin() {
        req.extensions_mut().insert(user);
        Ok(next.run(req).await)
    } else {
        Err(ResponseError::Forbidden(Some(String::from("You are forbidden to access this ressource"))))
    }
}