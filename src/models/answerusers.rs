use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Deserialize,Serialize,FromRow)]
pub struct AnswerUser {
    pub answer_id: i32,
    pub user_id: i32,
    pub form_id: i32,
    pub now: bool,
    pub commitment_pact: bool,
    pub comment: String,
    now_verif: bool,
    commitment_pact_verif: bool,
}

impl AnswerUser {
    pub fn new(
        answer_id: i32,
        user_id: i32,
        form_id: i32,
        now: bool,
        commitment_pact: bool,
        comment: String,
        now_verif: bool,
        commitment_pact_verif: bool,
    ) -> Self {
        Self {
            answer_id,
            user_id,
            form_id,
            now,
            commitment_pact,
            comment,
            now_verif,
            commitment_pact_verif,
        }
    }
    
}