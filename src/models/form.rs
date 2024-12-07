use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::models::question::Question;
use crate::models::answers::Answer;

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct Form {
    pub form_id: i32,
    pub company_id: i32,
    pub r#type: String,
}

#[derive(Deserialize, Debug)]
pub struct CreateForm {
    pub company_id: i32,
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FormWithQuestions {
    pub form: Form,
    pub questions: Vec<QuestionWithAnswers>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuestionWithAnswers {
    pub question: Question,
    pub answers: Vec<Answer>,
    pub user_answers: Vec<Answer>, // TODO : Change type to Vec<UserAnswer>
}

impl Form {
    pub fn new(
        form_id: i32,
        company_id: i32,
        r#type: String,
    ) -> Self {
        Self {
            form_id,
            company_id,
            r#type,
        }
    }
}

impl CreateForm {
    pub fn invalid(&self) -> bool {
        self.company_id <= 0 || self.r#type.is_empty()
    }
}