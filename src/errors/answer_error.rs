use axum::{http::StatusCode, response::IntoResponse};

pub enum AnswerError {
    BadRequest,
    DbError(sqlx::Error),
    NoSuchAnswer,
}

impl IntoResponse for AnswerError {
    fn into_response(self) -> axum::response::Response {
        let response = axum::http::Response::builder();
        let (code, message) = match self {
            AnswerError::DbError(e) => {
                println!("db error : {:?}", e); //tracing::warn!() plutot que println!() mais bon...
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            AnswerError::BadRequest => (StatusCode::BAD_REQUEST, "Bad request"),
            AnswerError::NoSuchAnswer => (StatusCode::NOT_FOUND, "No such answer"),
        };
        let body = axum::body::Body::from(message);
        response.status(code).body(body).unwrap()
    }
}