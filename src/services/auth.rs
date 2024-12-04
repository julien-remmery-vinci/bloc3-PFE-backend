use jsonwebtoken::{EncodingKey, Header};

use crate::models::{
    claims::Claims, 
    createuser::CreateUser, 
    credentials::Credentials, 
    user::User
};
use crate::errors::autherror::AuthError;

#[derive(Debug, Clone)]
pub struct AuthService {
    pub db: sqlx::PgPool,
}

impl AuthService {
    pub async fn login_user(&self, credentials: Credentials) -> Result<String, AuthError> {
        if credentials.invalid() {
            return Err(AuthError::BadRequest);
        }
        let user = User::find_by_login(&self.db, credentials.login.clone()).await
            .map_err(AuthError::DbError)?;
        if user.id == 0 {
            return Err(AuthError::NoSuchUser);
        }
        else if bcrypt::verify(&credentials.password, &user.password).map_err(AuthError::BCryptError)? {
            let token = create_token(credentials).map_err(AuthError::JWTError)?;
            return Ok(token)
        } else {
            return Err(AuthError::WrongPassword);
        }
    }

    pub async fn register_user(&self, mut user: CreateUser) -> Result<User, AuthError> {
        if user.invalid() {
            return Err(AuthError::BadRequest);
        }
        let found_user = User::find_by_login(&self.db, user.login.clone()).await
            .map_err(AuthError::DbError)?;
        if found_user.id != 0 {
            return Err(AuthError::Conflict);
        }
        let hashed_password = bcrypt::hash(&user.password, 12).map_err(AuthError::BCryptError)?;
        user.password = hashed_password;
        let created = User::create_user(&self.db, user).await.map_err(AuthError::DbError)?;
        Ok(created)
    }
}

fn create_token(credentials: Credentials) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims {
        sub: credentials.login,
        exp: (chrono::Utc::now() + chrono::Duration::days(1)).timestamp() as usize,
    };

    match jsonwebtoken::encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())) {
        Ok(token) => Ok(token),
        Err(error) => Err(error),
    }
}
