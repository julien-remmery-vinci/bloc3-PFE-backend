use sqlx::PgPool;

use crate::{errors::responserror::ResponseError, models::onboarding::Onboarding};

const QUERY_INSERT_ONBOARDING: &str = "
    INSERT INTO pfe.onboarding (
    firstname, lastname, email, password, position, company_name, company_number, legal_form, 
    office_address,  website, nace_code, revenue, franchise, nb_workers, dispute, honor_engagement,
    grant_application, grant_application_partner, something_else, comment,
    is_owner, offers_services, sells_products, status
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20,
    $21, $22, $23, 'PENDING') RETURNING *;
";
const QUERY_SELECT_ALL_ONBOARDING: &str = "
    SELECT * FROM pfe.onboarding;
";
const QUERY_SELECT_ALL_PENDING_ONBOARDING: &str = "
    SELECT * FROM pfe.onboarding WHERE status = 'PENDING';
";
const QUERY_SELECT_ALL_REJECTED_ONBOARDING: &str = "
    SELECT * FROM pfe.onboarding WHERE status = 'REJECTED';
";
const QUERY_ACCEPT_ONBOARDING: &str = "
    UPDATE pfe.onboarding SET status = 'ACCEPTED' WHERE onboarding_id = $1 RETURNING *;
";
const QUERY_REJECT_ONBOARDING: &str = "
    UPDATE pfe.onboarding SET status = 'REJECTED' WHERE onboarding_id = $1 RETURNING *;
";
const QUERY_SELECT_REQUEST_BY_EMAIL: &str = "
    SELECT * FROM pfe.onboarding WHERE email = $1 AND status != 'REJECTED';
";
const QUERY_SELECT_BY_ID: &str = "
    SELECT * FROM pfe.onboarding WHERE onboarding_id = $1;
";
const QUERY_SELECT_BY_COMPANY_NUMBER: &str = "
    SELECT * FROM pfe.onboarding WHERE company_number = $1 AND status != 'REJECTED';
";

#[derive(Debug, Clone)]
pub struct OnboardingService {
    pub db: PgPool,
}

impl OnboardingService {
    pub async fn submit_onboarding(&self, onboarding: Onboarding) -> Result<Onboarding, ResponseError> {
        match sqlx::query_as::<_, Onboarding>(QUERY_INSERT_ONBOARDING)
            .bind(onboarding.firstname)
            .bind(onboarding.lastname)
            .bind(onboarding.email)
            .bind(onboarding.password)
            .bind(onboarding.position)
            .bind(onboarding.company_name)
            .bind(onboarding.company_number)
            .bind(onboarding.legal_form)
            .bind(onboarding.office_address)
            .bind(onboarding.website)
            .bind(onboarding.nace_code)
            .bind(onboarding.revenue)
            .bind(onboarding.franchise)
            .bind(onboarding.nb_workers)
            .bind(onboarding.dispute)
            .bind(onboarding.honor_engagement)
            .bind(onboarding.grant_application)
            .bind(onboarding.grant_application_partner)
            .bind(onboarding.something_else)
            .bind(onboarding.comment)
            .bind(onboarding.is_owner)
            .bind(onboarding.offers_services)
            .bind(onboarding.sells_products)
            .fetch_one(&self.db)
            .await {
            Ok(onboarding) => Ok(onboarding),
            Err(e) => {
                tracing::error!("Error: {:?}", e);
                Err(ResponseError::DbError(e))
            }
        }
    }

    pub async fn read_by_id(&self, onboarding_id: i32) -> Result<Option<Onboarding>, ResponseError> {
        match sqlx::query_as::<_, Onboarding>(QUERY_SELECT_BY_ID)
            .bind(onboarding_id)
            .fetch_optional(&self.db)
            .await {
            Ok(onboarding) => Ok(onboarding),
            Err(e) => {
                tracing::error!("Error: {:?}", e);
                Err(ResponseError::DbError(e))
            }
        }
    }

    pub async fn read_request_by_email(&self, email: String) -> Result<Option<Onboarding>, ResponseError> {
        match sqlx::query_as::<_, Onboarding>(QUERY_SELECT_REQUEST_BY_EMAIL)
            .bind(email)
            .fetch_optional(&self.db)
            .await {
            Ok(onboarding) => Ok(onboarding),
            Err(e) => {
                tracing::error!("Error: {:?}", e);
                Err(ResponseError::DbError(e))
            }
        }
    }

    pub async fn read_all_onboarding(&self) -> Result<Vec<Onboarding>, ResponseError> {
        match sqlx::query_as::<_, Onboarding>(QUERY_SELECT_ALL_ONBOARDING)
            .fetch_all(&self.db)
            .await {
            Ok(onboarding) => Ok(onboarding),
            Err(e) => {
                tracing::error!("Error: {:?}", e);
                Err(ResponseError::DbError(e))
            }
        }
    }

    pub async fn read_all_pending_onboarding(&self) -> Result<Vec<Onboarding>, ResponseError> {
        match sqlx::query_as::<_, Onboarding>(QUERY_SELECT_ALL_PENDING_ONBOARDING)
            .fetch_all(&self.db)
            .await {
            Ok(onboarding) => Ok(onboarding),
            Err(e) => {
                tracing::error!("Error: {:?}", e);
                Err(ResponseError::DbError(e))
            }
        }
    }

    pub async fn read_all_rejected_onboarding(&self) -> Result<Vec<Onboarding>, ResponseError> {
        match sqlx::query_as::<_, Onboarding>(QUERY_SELECT_ALL_REJECTED_ONBOARDING)
            .fetch_all(&self.db)
            .await {
            Ok(onboarding) => Ok(onboarding),
            Err(e) => {
                tracing::error!("Error: {:?}", e);
                Err(ResponseError::DbError(e))
            }
        }
    }

    pub async fn accept_onboarding(&self, onboarding_id: i32) -> Result<Option<Onboarding>, ResponseError> {
        match sqlx::query_as::<_, Onboarding>(QUERY_ACCEPT_ONBOARDING)
            .bind(onboarding_id)
            .fetch_optional(&self.db)
            .await {
            Ok(onboarding) => Ok(onboarding),
            Err(e) => {
                tracing::error!("Error: {:?}", e);
                Err(ResponseError::DbError(e))
            }
        }
    }

    pub async fn reject_onboarding(&self, onboarding_id: i32) -> Result<Option<Onboarding>, ResponseError> {
        match sqlx::query_as::<_, Onboarding>(QUERY_REJECT_ONBOARDING)
            .bind(onboarding_id)
            .fetch_optional(&self.db)
            .await {
            Ok(onboarding) => Ok(onboarding),
            Err(e) => {
                tracing::error!("Error: {:?}", e);
                Err(ResponseError::DbError(e))
            }
        }
    }

    pub async fn read_by_company_number(&self, company_number: String) -> Result<Option<Onboarding>, ResponseError> {
        match sqlx::query_as::<_, Onboarding>(QUERY_SELECT_BY_COMPANY_NUMBER)
            .bind(company_number)
            .fetch_optional(&self.db)
            .await {
            Ok(onboarding) => Ok(onboarding),
            Err(e) => {
                tracing::error!("Error: {:?}", e);
                Err(ResponseError::DbError(e))
            }
        }
    }
}