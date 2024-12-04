use axum::{http::StatusCode, response::IntoResponse};

pub enum answerError {
    Conflict,
    BadRequest,
    DbError(sqlx::Error),
}

impl IntoResponse for answerError {
    fn into_response(self) -> axum::response::Response {
        let response = axum::http::Response::builder();
        let (code, message) = match self {
            answerError::DbError(e) => {
                println!("db error : {:?}", e); //tracing::warn!() plutot que println!() mais bon...
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            answerError::Conflict => (StatusCode::CONFLICT, "Answer already exists"),
            answerError::BadRequest => (StatusCode::BAD_REQUEST, "Bad request"),
        };
        let body = axum::body::Body::from(message);
        response.status(code).body(body).unwrap()
    }
}