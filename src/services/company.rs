use crate::{
    errors::responserror::ResponseError, 
    models::{company::Company, template::Template}
};

const QUERY_READ_BY_ID: &str = "SELECT * FROM pfe.companies WHERE company_id = $1";
const QUERY_READ_BY_COMPANY_NUMBER: &str = "SELECT * FROM pfe.companies WHERE company_number = $1";
const QUERY_GET_ALL_COMPANIES: &str = "
    SELECT company_id, company_name, company_number, legal_form, office_address, website,
    nace_code, nb_workers, revenue, dispute
    FROM pfe.companies
";
const QUERY_INSERT_COMPANY: &str = "
    INSERT INTO pfe.companies (company_name, company_number, legal_form, office_address, website,
    nace_code, nb_workers, revenue, dispute)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
    RETURNING *
";
const QUERY_SELECT_TEMPLATES: &str = "
    SELECT t.template_id, value 
    FROM pfe.templates t, pfe.template_company tc 
    WHERE t.template_id = tc.template_id AND tc.company_id = $1;
";
const QUERY_INSERT_COMPANY_TEMPLATES: &str = "
    INSERT INTO pfe.template_company (company_id, template_id)
    VALUES ($1, (SELECT template_id FROM pfe.templates WHERE value = $2))
    RETURNING *
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

    pub async fn read_by_company_number(&self, company_number: String) -> Result<Option<Company>, ResponseError> {
        let company = sqlx::query_as::<_, Company>(QUERY_READ_BY_COMPANY_NUMBER)
            .bind(company_number)
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

    pub async fn read_company_templates(&self, company_id: i32) -> Result<Vec<Template>, ResponseError> {
        let templates = sqlx::query_as::<_, Template>(QUERY_SELECT_TEMPLATES)
            .bind(company_id)
            .fetch_all(&self.db)
            .await.map_err(ResponseError::DbError)?;
        Ok(templates)
    }

    pub async fn create_company(&self, company: Company) -> Result<Company, ResponseError> {
        let company = sqlx::query_as::<_, Company>(QUERY_INSERT_COMPANY)
            .bind(company.company_name.clone())
            .bind(company.company_number.clone())
            .bind(company.legal_form.clone())
            .bind(company.office_address.clone())
            .bind(company.website.clone())
            .bind(company.nace_code.clone())
            .bind(company.nb_workers)
            .bind(company.revenue)
            .bind(company.dispute)
            .fetch_one(&self.db)
            .await.map_err(ResponseError::DbError)?;
        Ok(company)
    }

    pub async fn insert_company_templates(&self, company_id: i32, template: String) -> Result<(), ResponseError> {
        sqlx::query(QUERY_INSERT_COMPANY_TEMPLATES)
            .bind(company_id)
            .bind(template)
            .execute(&self.db)
            .await.map_err(ResponseError::DbError)?;
        Ok(())
    }
}