use axum::{http::StatusCode, response::IntoResponse};

pub enum GlobalError {
    Conflict,
    BadRequest,
    Unauthorized,
    Forbidden,
    InternalServerError,
    DbError(sqlx::Error)
}

impl IntoResponse for GlobalError {
    fn into_response(self) -> axum::response::Response {
        let response = axum::http::Response::builder();
        let (code, message) = match self {
            GlobalError::DbError(e) => {
                println!("db error : {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            GlobalError::Conflict => (StatusCode::CONFLICT, "User already exists"),
            GlobalError::BadRequest => (StatusCode::BAD_REQUEST, "Bad request"),
            GlobalError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            GlobalError::Forbidden => (StatusCode::FORBIDDEN, "Forbidden"),
            GlobalError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        };
        let body = axum::body::Body::from(message);
        response.status(code).body(body).unwrap()
    }
}