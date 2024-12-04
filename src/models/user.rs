use serde::Serialize;
use sqlx::FromRow;

use crate::models::createuser::CreateUser;

#[derive(Debug, FromRow, Clone, Serialize)]
pub struct User {
    pub id: i32,
    pub login: String,
    pub password: String,
}

const QUERY_READ_BY_EMAIL: &str = "SELECT id, login, password FROM pfe.users WHERE login = $1";
const QUERY_INSERT_USER: &str = "
            INSERT INTO pfe.users (login, password) 
            VALUES ($1, $2) 
            RETURNING id, login, password
        ";

impl User {
    pub async fn find_by_login(db: &sqlx::PgPool, login: String) -> Result<User, sqlx::error::Error> {
        match sqlx::query_as::<_, User>(QUERY_READ_BY_EMAIL)
            .bind(login)
            .fetch_all(db)
            .await
        {
            Ok(user) => {
                if user.is_empty() {
                    Ok(User::default())
                } else {
                    Ok(user[0].clone())
                }
            },
            Err(error) => Err(error),
        }
    }

    pub async fn create_user(db: &sqlx::PgPool, user: CreateUser) -> Result<User, sqlx::error::Error> {
        match sqlx::query_as::<_, User>(QUERY_INSERT_USER)
            .bind(user.login.clone())
            .bind(user.password.clone())
            .fetch_one(db)
            .await
        {
            Ok(user) => Ok(user),
            Err(error) => Err(error),
        }
    }

    fn default() -> User {
        User {
            id: 0,
            login: "".to_string(),
            password: "".to_string(),
        }
    }
}