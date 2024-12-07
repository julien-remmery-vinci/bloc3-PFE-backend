use axum::{http::StatusCode, response::IntoResponse};

pub enum CompanyError {
    BadRequest,
    DbError(sqlx::Error),
    NoSuchCompany,
}

impl IntoResponse for CompanyError {
    fn into_response(self) -> axum::response::Response {
        let response = axum::http::Response::builder();
        let (code, message) = match self {
            CompanyError::DbError(e) => {
                println!("db error : {:?}", e); //tracing::warn!() plutot que println!() mais bon...
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            CompanyError::BadRequest => (StatusCode::BAD_REQUEST, "Bad request"),
            CompanyError::NoSuchCompany => (StatusCode::NOT_FOUND, "No such company"),
        };
        let body = axum::body::Body::from(message);
        response.status(code).body(body).unwrap()
    }
}

