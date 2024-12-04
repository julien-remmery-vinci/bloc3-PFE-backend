use std::env;

use axum::body::Body;
use axum::extract::State;
use axum::http::{self, Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Header, TokenData, Validation};

use crate::database::state::AppState;
use crate::models::{
    claims::Claims, 
    createuser::CreateUser, 
    credentials::Credentials, 
    user::User
};
use crate::errors::autherror::AuthError;

#[derive(Debug, Clone)]
pub struct AuthService {
    pub db: sqlx::PgPool,
}

impl AuthService {
    pub async fn login_user(&self, credentials: Credentials) -> Result<String, AuthError> {
        if credentials.invalid() {
            return Err(AuthError::BadRequest);
        }
        let user = User::find_by_login(&self.db, credentials.login.clone()).await
            .map_err(AuthError::DbError)?;
        if user.id == 0 {
            return Err(AuthError::NoSuchUser);
        }
        else if bcrypt::verify(&credentials.password, &user.password).map_err(AuthError::BCryptError)? {
            let token = encode_jwt(credentials).map_err(AuthError::JWTError)?;
            return Ok(token)
        } else {
            return Err(AuthError::WrongPassword);
        }
    }

    pub async fn register_user(&self, mut user: CreateUser) -> Result<User, AuthError> {
        if user.invalid() {
            return Err(AuthError::BadRequest);
        }
        let found_user = User::find_by_login(&self.db, user.login.clone()).await
            .map_err(AuthError::DbError)?;
        if found_user.id != 0 {
            return Err(AuthError::Conflict);
        }
        let hashed_password = bcrypt::hash(&user.password, 12).map_err(AuthError::BCryptError)?;
        user.password = hashed_password;
        let created = User::create_user(&self.db, user).await.map_err(AuthError::DbError)?;
        Ok(created)
    }
}

fn encode_jwt(credentials: Credentials) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims {
        sub: credentials.login,
        exp: (chrono::Utc::now() + chrono::Duration::days(1)).timestamp() as usize,
    };
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    match jsonwebtoken::encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())) {
        Ok(token) => Ok(token),
        Err(error) => Err(error),
    }
}

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
pub async fn authorization_middleware(State(state): State<AppState>, mut req: Request<Body>, next: Next) -> Result<Response<Body>, AuthError> {
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

    println!("Token : {}", token);

    let token_data = match decode_jwt(token.to_string()) {
        Ok(data) => data,
        Err(_) => return Err(AuthError::TokenDecodeError),
    };

    let current_user = User::find_by_login(&state.auth.db, token_data.claims.sub).await
        .map_err(AuthError::DbError)?;
    if current_user.id == 0 {
        return Err(AuthError::Unauthorized);
    }

    req.extensions_mut().insert(current_user);
    Ok(next.run(req).await)
}