use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Stats {
    pub total_users: i64,
    pub total_companies: i64,
    pub total_forms: i64,
    pub total_questions: i64,
    pub total_answers: i64,
    pub total_templates_all: i64,
    pub total_templates_workers: i64,
    pub total_templates_owned_facility: i64,
    pub total_templates_products: i64,
    pub total_templates_facility: i64,
    pub total_onboarding: i64,
    pub total_accepted_onboarding: i64,
    pub total_rejected_onboarding: i64,
}