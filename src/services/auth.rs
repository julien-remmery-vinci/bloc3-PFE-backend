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
    user::User,
    tokenresponse::TokenResponse
};
use crate::errors::autherror::AuthError;

const QUERY_READ_BY_EMAIL: &str = "SELECT id, login, password FROM pfe.users WHERE login = $1";
const QUERY_INSERT_USER: &str = "
            INSERT INTO pfe.users (login, password) 
            VALUES ($1, $2) 
            RETURNING id, login, password
        ";

#[derive(Debug, Clone)]
pub struct AuthService {
    pub db: sqlx::PgPool,
}

impl AuthService {
    pub async fn find_by_login(&self, login: String) -> Result<User, sqlx::error::Error> {
        match sqlx::query_as::<_, User>(QUERY_READ_BY_EMAIL)
            .bind(login)
            .fetch_all(&self.db)
            .await
        {
            Ok(user) => {
                if user.is_empty() {
                    Ok(User::default())
                } else {
                    Ok(user[0].clone())
                }
            },
            Err(error) => Err(error),
        }
    }

    pub async fn create_user(&self, user: CreateUser) -> Result<User, sqlx::error::Error> {
        match sqlx::query_as::<_, User>(QUERY_INSERT_USER)
            .bind(user.login.clone())
            .bind(user.password.clone())
            .fetch_one(&self.db)
            .await
        {
            Ok(user) => Ok(user),
            Err(error) => Err(error),
        }
    }
}

pub fn encode_jwt(credentials: Credentials) -> Result<String, jsonwebtoken::errors::Error> {
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