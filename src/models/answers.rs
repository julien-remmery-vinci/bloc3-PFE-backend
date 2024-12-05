use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Deserialize, FromRow, Serialize)]
pub struct Answer {
    pub answer_id: i32,
    pub answer: String,
    pub template: String,
    pub question_id: i32,
    pub score: f64,
    pub engagement_score: f64,
    pub is_forced_engagement: bool,
    pub comment: String,
}

impl Answer {
    pub fn new(
        answer_id: i32,
        answer: String,
        template: String,
        question_id: i32,
        score: f64,
        engagement_score: f64,
        is_forced_engagement: bool,
        comment: String,
    ) -> Self {
        Self {
            answer_id,
            answer,
            template,
            question_id,
            score,
            engagement_score,
            is_forced_engagement,
            comment,
        }
    }
    
}