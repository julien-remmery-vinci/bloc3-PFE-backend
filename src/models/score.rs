use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Deserialize, FromRow, Serialize, Debug)]
pub struct ScoreQuery {
    pub sub_category: String,
    pub score: Option<f64>
}

impl ScoreQuery {
    pub fn new(sub_category: String, score: Option<f64>) -> Self {
        Self {
            sub_category,
            score
        }
    }
}