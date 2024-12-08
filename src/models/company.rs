use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Deserialize, FromRow, Serialize, Clone)]
pub struct Company {
    pub company_id: Option<i32>,
    pub company_name: String,
    pub company_number: String,
    pub office_address: String,
    pub legal_form: String,
    pub website: Option<String>,
    pub nace_code: String,
    pub business_activity: String,
    pub nb_workers: Option<i32>,
    pub revenue: Option<f64>,
    pub labels: Option<Vec<String>>,
    pub dispute: bool,
}