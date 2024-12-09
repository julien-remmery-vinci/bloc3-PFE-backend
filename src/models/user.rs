use serde::{
    Deserialize, 
    Serialize
};
use sqlx::FromRow;

#[derive(Debug, FromRow, Clone, Serialize)]
pub struct User {
    pub user_id: i32,
    pub firstname: String,
    pub lastname: String,
    pub login: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub role: String,
    pub company_id: Option<i32>,
}

impl User {  
    pub fn is_admin(&self) -> bool {
        self.role == "admin"
    }
}

#[derive(serde::Serialize)]
pub struct UserToken {
    pub user: User,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub firstname: String,
    pub lastname: String,
    pub login: String,
    pub password: String,
    pub role: String,
    pub company_id: Option<i32>,
}

impl CreateUser {
    pub fn invalid(&self) -> bool {
        self.firstname.is_empty() 
        || self.lastname.is_empty() 
        || self.login.is_empty() 
        || self.password.is_empty()
        || self.role.is_empty()
        || (self.role != "admin" && self.role != "user")
        || (self.company_id != None && self.company_id <= Some(0))
    }
}