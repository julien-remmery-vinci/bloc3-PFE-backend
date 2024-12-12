use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::errors::responserror::ResponseError;

#[derive(Deserialize, Serialize, FromRow, Debug, Clone)]
pub struct Question {
    pub question_id: i32,
    pub category: String,
    pub sub_category: String,
    pub question: String,
    pub is_used: bool,
    pub question_type: String,
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct QuestionRequest {
    pub category: String,
    pub sub_category: String,
    pub question: String,
    pub question_type: String,
    pub is_used: bool,
}

impl QuestionRequest {
    pub fn validate(&self) -> Result<(), ResponseError> {
        if self.question.is_empty()
            || self.category.is_empty()
            || self.sub_category.is_empty()
        {
            return Err(ResponseError::BadRequest(None));
        }
        Ok(())
    }
}

#[derive(Deserialize)]
pub struct PutQuestionRequest {
    pub question: Option<String>,
    pub is_used: Option<bool>,
}

impl PutQuestionRequest {
    pub fn update_validate(&self) -> Result<(), ResponseError> {
        if let Some(question) = &self.question {
            if question.is_empty() {
                return Err(ResponseError::BadRequest(None));
            }
        }
        Ok(())
    }
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct QuestionForm {
    pub form_id: i32,
    pub question_id: i32,
    pub question_status: String,
}