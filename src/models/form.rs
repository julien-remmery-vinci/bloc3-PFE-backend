use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::models::question::Question;
use crate::models::answers::Answer;
use serde_json::Value;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Form {
    pub form_id: Option<i32>,
    pub company: i32,
    pub form_type: String,
    pub nb_questions: i32,
    pub template: String,
    pub questions: Option<Value>,
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
        nb_questions: i32,
        template: String,
        questions: Option<Vec<QuestionWithAnswers>>,
    ) -> Self {
        Self {
            form_id: Some(form_id),
            company,
            form_type,
            nb_questions,
            template,
            questions: questions.map(|q| serde_json::to_value(q).expect("Failed to serialize questions")),
        }
    }

    pub fn get_questions(&self) -> Option<Vec<QuestionWithAnswers>> {
        self.questions.as_ref().map(|json| {
            serde_json::from_value(json.clone()).expect("Failed to deserialize questions")
        })
    }
}