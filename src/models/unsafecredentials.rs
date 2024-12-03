use serde::Deserialize;

#[derive(Deserialize)]
pub struct UnsafeCredentials {
    pub login: String,
    pub password: String,
}

impl UnsafeCredentials {
    pub fn new(login: String, password: String) -> Self {
        Self {
            login,
            password,
        }
    }
}
