use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Deserialize, FromRow, Serialize, Debug)]
pub struct Answer {
    pub answer_id: i32,
    pub question_id: Option<i32>,
    pub template: String,
    pub answer: String,
    pub score_now: f64,
    pub score_commitment_pact: f64,
    pub is_forced_engagement: bool,
}

impl Answer {
    pub fn new(
        answer_id: i32,
        question_id: Option<i32>,
        template: String,
        answer: String,
        score_now: f64,
        score_commitment_pact: f64,
        is_forced_engagement: bool,
    ) -> Self {
        Self {
            answer_id,
            question_id,
            template,
            answer,
            score_now,
            score_commitment_pact,
            is_forced_engagement,
        }
    }
    
}