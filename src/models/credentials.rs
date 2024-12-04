use serde::Deserialize;

#[derive(Deserialize)]
pub struct Credentials {
    pub login: String,
    pub password: String,
}

impl Credentials {
    pub fn invalid(&self) -> bool {
        self.login.is_empty() || self.password.is_empty()
    }
}
