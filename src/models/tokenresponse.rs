use serde::Serialize;

#[derive(Serialize)]
pub struct TokenResponse {
    pub token: String,
}