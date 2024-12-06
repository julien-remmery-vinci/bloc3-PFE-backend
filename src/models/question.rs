use serde::Deserialize;

#[derive(Deserialize)]
pub struct Question {
    pub id: i32,
    pub category: String,
    pub sub_category: String,
    pub question: String,
    pub is_used: bool,
}

impl Question {
    pub fn new(
        id: i32,
        category: String,
        sub_category: String,
        question: String,
        is_used: bool,
    ) -> Self {
        Self {
            id,
            category,
            sub_category,
            question,
            is_used,
        }
    }
}
