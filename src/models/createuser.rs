use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CreateUser {
    pub login: String,
    pub password: String,
}

impl CreateUser {
    pub fn invalid(&self) -> bool {
        self.login.is_empty() || self.password.is_empty()
    }
}