use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, FromRow, Clone, Serialize)]
pub struct User {
    pub id: i32,
    pub login: String,
    pub password: String,
    pub role: String,
}

impl User {
    pub fn default() -> User {
        User {
            id: 0,
            login: String::from(""),
            password: String::from(""),
            role: String::from(""),
        }
    }
    
    pub fn is_admin(&self) -> bool {
        self.role == "admin"
    }
}