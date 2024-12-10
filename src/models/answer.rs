use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Deserialize, FromRow, Serialize, Debug)]
pub struct Answer {
    pub answer_id: i32,
    pub question_id: Option<i32>,
    pub template: String,
    pub answer: Option<String>,
    pub score_now: f64,
    pub score_commitment_pact: f64,
    pub is_forced_engagement: bool,
    pub is_forced_comment: bool,
}

#[derive(Deserialize,Serialize,FromRow, Debug, Clone)]
pub struct AnswerUser {
    pub answer_id: i32,
    pub user_id: i32,
    pub form_id: i32,
    pub now: Option<bool>,
    pub commitment_pact: Option<bool>,
    pub comment: Option<String>,
    pub now_verif: Option<bool>,
    pub commitment_pact_verif: Option<bool>,
    pub status: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateAnswer {
    pub answer: String,
    pub template: String,
    pub question_id: i32,
    pub score: f64,
    pub engagement_score: f64,
    pub is_forced_engagement: bool,
    pub is_forced_comment: bool,
}

#[derive(Deserialize)]
pub struct ValidatedAnswer {
    pub form_id: i32,
    pub comment: Option<String>,
    pub now_verif: Option<bool>,
    pub commitment_pact_verif: Option<bool>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CreateAnswerUser {
    pub form_id: i32,
    pub now: Option<bool>,
    pub commitment_pact: Option<bool>,
    pub comment: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CreateAnswerValidation {
    pub answer_id: i32,
    pub form_id: i32,
    pub user_id: i32,
    pub comment: Option<String>,
    pub now_verif: Option<bool>,
    pub commitment_pact_verif: Option<bool>,
}

impl CreateAnswerUser {
    pub fn invalid(&self) -> bool {
        self.form_id <= 0
    }
}

impl CreateAnswer {
    pub fn invalid(&self) -> bool {
        self.answer.is_empty()
            || self.template.is_empty()
            || self.question_id == 0
            || self.score < 0.0
            || self.engagement_score < 0.0
    }
}