use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::models::question::Question;
use crate::models::answers::Answer;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Form {
    pub form_id: Option<i32>,
    pub company: i32,
    pub form_type: String,
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
}

impl Form {
    pub fn new(
        form_id: i32,
        company: i32,
        form_type: String,
    ) -> Self {
        Self {
            form_id: Some(form_id),
            company,
            form_type,
        }
    }
}

impl FormWithQuestions {
    pub fn get_questions(&self) -> &Vec<QuestionWithAnswers> {
        &self.questions
    }
}