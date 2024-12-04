use axum::{http::StatusCode, response::IntoResponse};

pub enum AuthError {
    Conflict,
    BadRequest,
    WrongPassword,
    Unauthorized,
    NoSuchUser,
    DbError(sqlx::Error),
    BCryptError(bcrypt::BcryptError),
    JWTError(jsonwebtoken::errors::Error),
    EmptyHeaderError,
    NoTokenError,
    TokenDecodeError
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
            AuthError::Conflict => (StatusCode::CONFLICT, "User already exists"),
            AuthError::BadRequest => (StatusCode::BAD_REQUEST, "Bad request"),
            AuthError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            AuthError::Forbidden => (StatusCode::FORBIDDEN, "Forbidden"),
            AuthError::WrongPassword => (StatusCode::UNAUTHORIZED, "Wrong password"),
            AuthError::NoSuchUser => (StatusCode::NOT_FOUND, "User not found"),
            AuthError::EmptyHeaderError => (StatusCode::BAD_REQUEST, "Empty header is not allowed"),
            AuthError::NoTokenError => (StatusCode::UNAUTHORIZED, "Please add the JWT token to the header"),
            AuthError::TokenDecodeError => (StatusCode::UNAUTHORIZED, "Unable to decode token")
        };
        let body = axum::body::Body::from(message);
        response.status(code).body(body).unwrap()
    }
}