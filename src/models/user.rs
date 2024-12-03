
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
}

impl User {
    pub fn new(id: u32, username: String, email: String) -> Self {
        User { id, username, email }
    }
}