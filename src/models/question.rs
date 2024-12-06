use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use super::form;

#[derive(Deserialize, Serialize, FromRow, Debug)]
pub struct Question {
    pub id: i32,
    pub form_id: Option<i32>,
    pub category: String,
    pub sub_category: String,
    pub question: String,
    pub is_used: bool,
}

impl Question {
    pub fn new(
        id: i32,
        form_id: i32,
        category: String,
        sub_category: String,
        question: String,
        is_used: bool,
    ) -> Self {
        Self {
            id,
            form_id: Some(form_id),
            category,
            sub_category,
            question,
            is_used,
        }
    }
}
