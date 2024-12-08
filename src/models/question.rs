use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Deserialize, Serialize, FromRow, Debug, Clone)]
pub struct Question {
    pub question_id: i32,
    pub category: String,
    pub sub_category: String,
    pub question: String,
    pub is_used: bool,
}

impl Question {
    pub fn new(
        question_id: i32,
        category: String,
        sub_category: String,
        question: String,
        is_used: bool,
    ) -> Self {
        Self {
            question_id,
            category,
            sub_category,
            question,
            is_used,
        }
    }
}
