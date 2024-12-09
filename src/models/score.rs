use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Deserialize, FromRow, Serialize, Debug,Clone)]
pub struct ScoreQuery {
    pub sub_category: String,
    pub score: Option<f64>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Score {
    pub total: f64,
    pub details_now: Vec<ScoreQuery>,
    pub details_commitment_pact: Vec<ScoreQuery>,
}