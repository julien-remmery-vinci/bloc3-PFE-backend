use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct Onboarding {
    pub onboarding_id: Option<i32>,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
    pub position: String,
    pub company_name: String,
    pub company_number: String,
    pub legal_form: String,
    pub office_address: String,
    pub website: String,
    pub nace_code: String,
    pub revenue: f64,
    pub franchise: bool,
    pub nb_workers: i32,
    pub dispute: bool,
    pub honor_engagement: bool,
    pub grant_application: bool,
    pub grant_application_partner: String,
    pub something_else: String,
    pub comment: String,
    pub submit_date: Option<String>,
    pub is_owner: bool,
    pub offers_services: bool,
    pub sells_products: bool,
    pub status: Option<String>,
}

impl Onboarding {
    pub fn invalid(&self) -> bool {
        self.firstname.is_empty()
            || self.lastname.is_empty()
            || self.email.is_empty()
            || self.password.is_empty()
            || self.position.is_empty()
            || self.company_name.is_empty()
            || self.company_number.is_empty()
            || self.legal_form.is_empty()
            || self.office_address.is_empty()
            || self.website.is_empty()
            || self.nace_code.is_empty()
            || self.revenue == 0.0
            || self.nb_workers == 0
            || self.honor_engagement == false
    }

    pub fn _pending(&self) -> bool {
        if self.status.is_none() {
            return false;
        }
        self.status.as_ref().unwrap() == "PENDING"
    }

    pub fn rejected(&self) -> bool {
        if self.status.is_none() {
            return false;
        }
        self.status.as_ref().unwrap() == "REJECTED"
    }

    pub fn accepted(&self) -> bool {
        if self.status.is_none() {
            return false;
        }
        self.status.as_ref().unwrap() == "ACCEPTED"
    }
}