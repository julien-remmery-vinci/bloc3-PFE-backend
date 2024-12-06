use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Deserialize, FromRow, Serialize, Debug)]
pub struct ScoreQuery {
    pub form_id: i32,
}

impl ScoreQuery {
    pub fn new(form_id: i32) -> Self {
        Self { form_id }
    }
}