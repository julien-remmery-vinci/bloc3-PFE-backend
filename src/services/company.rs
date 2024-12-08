use crate::{
    errors::globalerror::ResponseError, 
    models::company::Company
};

const QUERY_READ_BY_ID: &str = "SELECT * FROM pfe.companies WHERE company_id = $1";

const QUERY_GET_ALL_COMPANIES: &str = "
    SELECT company_id, company_name, company_number, legal_form, office_address, website,
    nace_code, business_activity, nb_workers, revenue, labels, dispute
    FROM pfe.companies
";


#[derive(Debug, Clone)]
pub struct CompanyService {
    pub db: sqlx::PgPool,
}

impl CompanyService {
    pub async fn find_by_id(&self, company_id: i32) -> Result<Option<Company>, ResponseError> {
        let company = sqlx::query_as::<_, Company>(QUERY_READ_BY_ID)
            .bind(company_id)
            .fetch_optional(&self.db)
            .await
            .map_err(ResponseError::DbError)?;
        Ok(company)
    }

    pub async fn get_companies(&self) -> Result<Vec<Company>, ResponseError> {
        match sqlx::query_as::<_, Company>(QUERY_GET_ALL_COMPANIES)
            .fetch_all(&self.db)
            .await
        {
            Ok(companies) => Ok(companies),
            Err(error) => Err(ResponseError::DbError(error)),
        }
    }

    // pub async fn create_company(&self, company: CreateCompany) -> Result<Company, sqlx::error::Error> {
    //     match sqlx::query_as::<_, Company>(QUERY_INSERT_COMPANY)
    //         .bind(company.name.clone())
    //         .bind(company.address.clone())
    //         .bind(company.city.clone())
    //         .bind(company.zip_code.clone())
    //         .bind(company.country.clone())
    //         .fetch_one(&self.db)
    //         .await
    //     {
    //         Ok(company) => Ok(company),
    //         Err(error) => Err(error),
    //     }
    // }
}