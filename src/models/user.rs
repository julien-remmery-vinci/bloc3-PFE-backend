use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub login: String,
    pub password: String,
}

impl User {
    pub fn new(id: i32, login: String, password: String) -> Self {
        User { id, login, password }
    }
}