use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::models::question::Question;
use crate::models::answer::Answer;

use super::answer::AnswerUser;
use super::template::Template;

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct Form {
    pub form_id: i32,
    pub company_id: i32,
    pub r#type: String,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct CompleteForm {
    pub form_id: i32,
    pub company_id: i32,
    pub r#type: String,
    pub templates: Vec<Template>,
    pub questions: Vec<QuestionWithAnswers>,
}

#[derive(Deserialize, Debug)]
pub struct CreateForm {
    pub company_id: i32,
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuestionWithAnswers {
    pub question: Question,
    pub answers: Vec<Answer>,
    pub user_answers: Vec<AnswerUser>,
}

impl CreateForm {
    pub fn invalid(&self) -> bool {
        self.company_id <= 0 || self.r#type.is_empty()
    }
}