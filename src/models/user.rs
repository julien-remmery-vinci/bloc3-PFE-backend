use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, FromRow, Clone, Serialize)]
pub struct User {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub login: String,
    pub password: String,
    pub role: String,
    pub company_id: Option<i32>,
}

impl User {
    pub fn default() -> User {
        User {
            id: 0,
            firstname: String::from(""),
            lastname: String::from(""),
            login: String::from(""),
            password: String::from(""),
            role: String::from(""),
            company_id: None,
        }
    }
    
    pub fn is_admin(&self) -> bool {
        self.role == "admin"
    }
}