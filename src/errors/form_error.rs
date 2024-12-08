use axum::{http::StatusCode, response::IntoResponse};

pub enum FormError {
    BadRequest,
    NotFound,
    DbError(sqlx::Error),
}

impl IntoResponse for FormError {
    fn into_response(self) -> axum::response::Response {
        let response = axum::http::Response::builder();
        let (code, message) = match self {
            FormError::DbError(e) => {
                println!("form db error : {:?}", e); //tracing::warn!() plutôt que println!() mais bon...
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            FormError::BadRequest => (StatusCode::BAD_REQUEST, "Bad request"),
            FormError::NotFound => (StatusCode::NOT_FOUND, "Form not found"),
        };
        let body = axum::body::Body::from(message);
        response.status(code).body(body).unwrap()
    }
}