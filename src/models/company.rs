use serde::Deserialize;

#[derive(Deserialize)]
pub struct Company {
    pub company_id: i32,
    pub company_name: String,
    pub comppany_number: String,
    pub company_address: String,
    pub legal_form: String,
    pub website: Option<String>,
    pub nace_code: String,
    pub business_activity: String,
    pub nb_employees: Option<i32>,
    pub revenue: Option<f64>,
    pub labels: Option<String>,
    pub dispute: bool,
}

impl Company {
    pub fn new(company_id: i32, company_name: String, comppany_number: String, company_address: String, legal_form: String, website: Option<String>, nace_code: String, business_activity: String, nb_employees: Option<i32>, revenue: Option<f64>, labels: Option<String>, dispute: bool) -> Self {
        Self {
            company_id,
            company_name,
            comppany_number,
            company_address,
            legal_form,
            website,
            nace_code,
            business_activity,
            nb_employees,
            revenue,
            labels,
            dispute,
        }
    }
}