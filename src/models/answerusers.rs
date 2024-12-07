use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Deserialize,Serialize,FromRow)]
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

impl AnswerUser {
    pub fn new(
        answer_id: i32,
        user_id: i32,
        form_id: i32,
        answer: Option<String>,
        now: bool,
        commitment_pact: bool,
        comment: String,
        now_verif: Option<bool>,
        commitment_pact_verif: Option<bool>,
    ) -> Self {
        Self {
            answer_id,
            user_id,
            form_id,
            answer,
            now,
            commitment_pact,
            comment,
            now_verif,
            commitment_pact_verif,
        }
    }
    
}