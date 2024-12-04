use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Serialize;
use jsonwebtoken::{EncodingKey, Header};

use crate::{
    models::credentials::Credentials,
    models::user::User,
    models::claims::Claims,
};

#[derive(Debug, Clone)]
pub struct AuthService {
    pub db: sqlx::PgPool,
}

const LOGIN_QUERY: &str = "SELECT id, login, password FROM pfe.users WHERE login = $1";


impl AuthService {
    pub async fn auth_query(&self, credentials: Credentials) -> Result<OkResponse, AuthError> {
        if credentials.invalid() {
            return Err(AuthError::BadRequest);
        }
        match sqlx::query_as::<_, User>(LOGIN_QUERY)
            .bind(credentials.login.clone())
            .fetch_one(&self.db)
            .await
        {
            Ok(user) => {
                if user.login != credentials.login {
                    Err(AuthError::NoSuchUser)
                } else if bcrypt::verify(&credentials.password, &user.password).map_err(AuthError::BCryptError)?
                {
                    let token = create_token(credentials).map_err(AuthError::JWTError)?;
                    Ok(OkResponse { token })
                } else {
                    Err(AuthError::WrongPassword)
                }
            }
            Err(error) => Err(AuthError::DbError(error)),
        }
    }
}

pub enum AuthError {
    BadRequest,
    WrongPassword,
    NoSuchUser,
    DbError(sqlx::Error),
    BCryptError(bcrypt::BcryptError),
    JWTError(jsonwebtoken::errors::Error),
}

#[derive(Serialize)]
pub struct OkResponse {
    token: String,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let response = axum::http::Response::builder();
        let (code, message) = match self {
            AuthError::DbError(e) => {
                println!("db error : {:?}", e); //tracing::warn!() plutot que println!() mais bon...
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            AuthError::BCryptError(e) => {
                println!("encryption error : {:?}", e); //tracing::warn!() plutot que println!() mais bon...
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            AuthError::JWTError(e) => {
                println!("jwt error : {:?}", e); //tracing::warn!() plutot que println!() mais bon...
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            AuthError::BadRequest => (StatusCode::BAD_REQUEST, "Bad request"),
            AuthError::WrongPassword => (StatusCode::UNAUTHORIZED, "Wrong password"),
            AuthError::NoSuchUser => (StatusCode::NOT_FOUND, "User not found"),
        };
        let body = axum::body::Body::from(message);
        response.status(code).body(body).unwrap()
    }
}

fn create_token(credentials: Credentials) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims {
        sub: credentials.login,
        exp: (chrono::Utc::now() + chrono::Duration::days(1)).timestamp() as usize,
    };

    match jsonwebtoken::encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())) {
        Ok(token) => Ok(token),
        Err(error) => Err(error),
    }
}
