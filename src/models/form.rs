use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Form {
    pub form_id: Option<i32>,
    pub company: i32,
    pub form_type: String,
    pub nb_questions: i32,
    pub template: String,
}

impl Form {
    pub fn new(form_id: i32, company: i32, form_type: String, nb_questions: i32, template: String) -> Self {
        Self {
            form_id: Some(form_id),
            company,
            form_type,
            nb_questions,
            template,
        }
    }
}