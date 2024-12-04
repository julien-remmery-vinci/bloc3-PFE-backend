use serde::Deserialize;

#[derive(Deserialize)]
pub struct Question {
    pub id: i32,
    pub question_status: String,
    pub category: String,
    pub sub_category: String,
    pub question: String,
}

impl Question {
    pub fn new(
        id: i32,
        question_status: String,
        category: String,
        sub_category: String,
        question: String,
    ) -> Self {
        Self {
            id,
            question_status,
            category,
            sub_category,
            question,
        }
    }
}
