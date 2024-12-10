use std::env;
use jsonwebtoken::{EncodingKey, Header};
use crate::{errors::responserror::ResponseError, models::{
    claims::Claims, 
    user::CreateUser, 
    credentials::Credentials, 
    user::User
}};

const QUERY_READ_BY_EMAIL: &str = "
            SELECT user_id, firstname, lastname, login, password, role, company_id
            FROM pfe.users 
            WHERE login = $1";
const QUERY_INSERT_USER: &str = "
            INSERT INTO pfe.users (firstname, lastname, login, password, role, company_id) 
            VALUES ($1, $2, $3, $4, $5, $6) 
            RETURNING *
        ";

#[derive(Debug, Clone)]
pub struct AuthService {
    pub db: sqlx::PgPool,
}

impl AuthService {
    pub async fn find_by_login(&self, login: String) -> Result<Option<User>, ResponseError> {
        let user = sqlx::query_as::<_, User>(QUERY_READ_BY_EMAIL)
            .bind(login)
            .fetch_optional(&self.db)
            .await.map_err(ResponseError::DbError)?;
        Ok(user)
    }

    pub async fn create_user(&self, user: CreateUser) -> Result<User, ResponseError> {
        match sqlx::query_as::<_, User>(QUERY_INSERT_USER)
            .bind(user.firstname.clone())
            .bind(user.lastname.clone())
            .bind(user.login.clone())
            .bind(user.password.clone())
            .bind(user.role.clone())
            .bind(user.company_id)
            .fetch_one(&self.db)
            .await
        {
            Ok(user) => Ok(user),
            Err(error) => Err(ResponseError::DbError(error)),
        }
    }
}

pub fn encode_jwt(credentials: Credentials) -> Result<String, ResponseError> {
    let claims = Claims {
        sub: credentials.login,
        exp: (chrono::Utc::now() + chrono::Duration::days(1)).timestamp() as usize,
    };
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    match jsonwebtoken::encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())) {
        Ok(token) => Ok(token),
        Err(error) => Err(ResponseError::JWTError(error)),
    }
}

pub fn hash_password(password: String) -> Result<String, ResponseError> {
    let hashed_password = bcrypt::hash(password, env::var("HASH_ROUNDS")
        .expect("HASH_ROUNDS must be set")
        .parse::<u32>()
        .unwrap())
        .map_err(ResponseError::BCryptError)?;
    Ok(hashed_password)
}