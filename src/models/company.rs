use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use super::form::CompleteForm;

#[derive(Deserialize, FromRow, Serialize, Clone)]
pub struct Company {
    pub company_id: Option<i32>,
    pub company_name: String,
    pub company_number: String,
    pub office_address: String,
    pub legal_form: String,
    pub website: Option<String>,
    pub nace_code: String,
    pub nb_workers: Option<i32>,
    pub revenue: Option<f64>,
    pub dispute: bool,
}

#[derive(Deserialize, FromRow, Serialize)]
pub struct CompanyWithCompleteForms {
    pub company: Company,
    pub forms: Vec<CompleteForm>,
}

#[derive(Deserialize, Serialize)]
pub struct CompanyValidation {
    pub is_eligible: bool,
}

impl Company {
    pub fn invalid (&self) -> bool {
        self.company_name.is_empty() 
        || self.company_number.is_empty() 
        || self.office_address.is_empty() 
        || self.legal_form.is_empty() 
        || self.nace_code.is_empty()
    }
}