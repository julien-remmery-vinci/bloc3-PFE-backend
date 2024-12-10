use axum::{
    http::StatusCode, 
    response::IntoResponse
};

#[derive(Debug)]
pub enum ResponseError {
    Conflict(Option<String>),
    BadRequest(Option<String>),
    Unauthorized(Option<String>),
    Forbidden(Option<String>),
    NotFound(Option<String>),
    DbError(sqlx::Error),
    BCryptError(bcrypt::BcryptError),
    JWTError(jsonwebtoken::errors::Error),
}

impl IntoResponse for ResponseError {
    fn into_response(self) -> axum::response::Response {
        let response = axum::http::Response::builder();
        let (code, message) = match self {
            ResponseError::Conflict(str) => (
                StatusCode::CONFLICT, 
                str.unwrap_or_else(|| String::from("Conflict"))
            ),
            ResponseError::BadRequest(str) => (
                StatusCode::BAD_REQUEST,
                str.unwrap_or_else(|| String::from("Bad request"))
            ),
            ResponseError::Unauthorized(str) => (
                StatusCode::UNAUTHORIZED, 
                str.unwrap_or_else(|| String::from("Unauthorized"))
            ),
            ResponseError::Forbidden(str) => (
                StatusCode::FORBIDDEN, 
                str.unwrap_or_else(|| String::from("Forbidden"))
            ),
            ResponseError::NotFound(str) => (
                StatusCode::NOT_FOUND, 
                str.unwrap_or_else(|| String::from("Not found"))
            ),
            ResponseError::DbError(e) => {
                tracing::error!("Database error : {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, String::from("Internal server error"))
            },
            ResponseError::BCryptError(e) => {
                tracing::error!("Bcrypt error : {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, String::from("Internal server error"))
            }
            ResponseError::JWTError(e) => {
                tracing::error!("JWT error : {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, String::from("Internal server error"))
            }
        };
        let body = axum::body::Body::from(message);
        response.status(code).body(body).unwrap()
    }
}