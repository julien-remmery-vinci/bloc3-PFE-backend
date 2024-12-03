use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CreateUser {
    pub login: String,
    pub password: String,
}

impl CreateUser {
    pub fn new(login: String, password: String) -> Self {
        CreateUser { login, password }
    }
}