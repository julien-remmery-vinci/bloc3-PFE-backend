use crate::{errors::company_error::CompanyError, models::company::Company};

const QUERY_READ_BY_ID: &str = "SELECT * FROM pfe.companies WHERE company_id = $1";
// const QUERY_INSERT_COMPANY: &str = "
//             INSERT INTO company (name, address, city, zip_code, country) 
//             VALUES ($1, $2, $3, $4, $5) 
//             RETURNING *";

const QUERY_GET_ALL_COMPANIES: &str = "SELECT company_id, company_name, company_number, legal_form, office_address, website,
                                        nace_code, business_activity, nb_workers, revenue, labels, dispute
                                        FROM pfe.companies";


#[derive(Debug, Clone)]
pub struct CompanyService {
    pub db: sqlx::PgPool,
}

impl CompanyService {
    pub async fn find_by_id(&self, id: i32) -> Result<Company, sqlx::error::Error> {
        match sqlx::query_as::<_, Company>(QUERY_READ_BY_ID)
            .bind(id)
            .fetch_all(&self.db)
            .await
        {
            Ok(result) => {
                if result.is_empty() {
                    Ok(Company::default())
                } else {
                    Ok(result[0].clone())
                }
            },
            Err(error) => Err(error),
        }
    }

    pub async fn get_companies(&self) -> Result<Vec<Company>, CompanyError> {
        match sqlx::query_as::<_, Company>(QUERY_GET_ALL_COMPANIES)
            .fetch_all(&self.db)
            .await
        {
            Ok(companies) => Ok(companies),
            Err(error) => Err(CompanyError::DbError(error)),
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