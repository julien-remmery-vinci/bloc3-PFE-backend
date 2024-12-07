use axum::{http::StatusCode, response::IntoResponse};

pub enum QuestionError {
    BadRequest,
    DbError(sqlx::Error),
    NoSuchQuestion,
}

impl IntoResponse for QuestionError {
    fn into_response(self) -> axum::response::Response {
        let response = axum::http::Response::builder();
        let (code, message) = match self {
            QuestionError::DbError(e) => {
                println!("question db error : {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            QuestionError::BadRequest => (StatusCode::BAD_REQUEST, "Bad request"),
            QuestionError::NoSuchQuestion => (StatusCode::NOT_FOUND, "Question not found"),
        };
        let body = axum::body::Body::from(message);
        response.status(code).body(body).unwrap()
    }
}