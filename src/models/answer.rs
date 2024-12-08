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
}

#[derive(Deserialize,Serialize,FromRow, Debug)]
pub struct AnswerUser {
    pub answer_id: i32,
    pub user_id: i32,
    pub form_id: i32,
    pub answer: Option<String>,
    pub now: bool,
    pub commitment_pact: bool,
    pub comment: String,
    pub now_verif: Option<bool>,
    pub commitment_pact_verif: Option<bool>,
}

#[derive(Deserialize)]
pub struct CreateAnswer {
    pub answer: String,
    pub template: String,
    pub question_id: i32,
    pub score: f64,
    pub engagement_score: f64,
    pub is_forced_engagement: bool,
}

#[derive(Deserialize)]
pub struct CreateAnswerUser {
    pub answer: Option<String>,
    pub form_id: i32,
    pub now: bool,
    pub commitment_pact: bool,
    pub comment: String,
}