use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize,Clone)]
pub struct UnsafeCredentials {
    pub login: String,
    pub password: String,
}

#[derive(Serialize,Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

impl UnsafeCredentials {
    pub fn new(login: String, password: String) -> Self {
        Self {
            login,
            password,
        }
    }

    pub fn invalid(&self) -> bool {
        self.login.is_empty() || self.password.is_empty()
    }
}
