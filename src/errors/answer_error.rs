use axum::{http::StatusCode, response::IntoResponse};

pub enum AnswerError {
    Conflict,
    BadRequest,
    DbError(sqlx::Error),
}

impl IntoResponse for AnswerError {
    fn into_response(self) -> axum::response::Response {
        let response = axum::http::Response::builder();
        let (code, message) = match self {
            AnswerError::DbError(e) => {
                println!("db error : {:?}", e); //tracing::warn!() plutot que println!() mais bon...
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            AnswerError::Conflict => (StatusCode::CONFLICT, "Answer already exists"),
            AnswerError::BadRequest => (StatusCode::BAD_REQUEST, "Bad request"),
        };
        let body = axum::body::Body::from(message);
        response.status(code).body(body).unwrap()
    }
}