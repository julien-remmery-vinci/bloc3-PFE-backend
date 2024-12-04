use axum::{http::StatusCode, response::IntoResponse};

pub enum QuestionError {
    BadRequest,
    DbError(sqlx::Error),
}

impl IntoResponse for QuestionError {
    fn into_response(self) -> axum::response::Response {
        let response = axum::http::Response::builder();
        let (code, message) = match self {
            QuestionError::DbError(e) => {
                println!("db error : {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            QuestionError::BadRequest => (StatusCode::BAD_REQUEST, "Bad request"),
        };
        let body = axum::body::Body::from(message);
        response.status(code).body(body).unwrap()
    }
}