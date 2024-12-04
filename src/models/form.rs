use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Form {
    pub form_id: i32,
    pub company_id: i32,
    pub form_type: String,
    pub nb_questions: i32,
    pub template: String,
}

impl Form {
    pub fn new(form_id: i32, company_id: i32, form_type: String, nb_questions: i32, template: String) -> Self {
        Self {
            form_id,
            company_id,
            form_type,
            nb_questions,
            template,
        }
    }
}