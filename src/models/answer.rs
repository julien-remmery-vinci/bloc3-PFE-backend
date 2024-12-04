use serde::Deserialize;

#[derive(Deserialize)]
pub struct Answer {
    pub id: i32,
    pub answer: String,
    pub template: String,
    pub question_id: i32,
    pub score: i32,
    pub engagement_score: i32,
    pub is_forced_engagement: bool,
    pub comment: String,
}

impl Answer {
    pub fn new(id: i32, answer: String, template: String, question_id: i32, score: i32, engagement_score: i32, is_forced_engagement: bool, comment: String) -> Self {
        Self {
            id,
            answer,
            template,
            question_id,
            score,
            engagement_score,
            is_forced_engagement,
            comment,
        }
    }

    pub fn invalid(&self) -> bool {
        self.answer.is_empty() || self.template.is_empty() || self.comment.is_empty()
    }
    
}