use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CreateAnswer {
    pub id: i32,
    pub answer: String,
    pub template: String,
    pub question_id: i32,
    pub score: f64,
    pub engagement_score: f64,
    pub is_forced_engagement: bool,
    pub comment: String,
}

impl CreateAnswer {
    pub fn invalid(&self) -> bool {
        self.answer.is_empty() || self.template.is_empty() || self.question_id == 0 || self.score < 0.0 || self.engagement_score < 0.0 || self.comment.is_empty() 
    }
}