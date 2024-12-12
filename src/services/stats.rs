use sqlx::{Pool, Postgres};

use crate::{errors::responserror::ResponseError, models::stats::Stats};

const QUERY_SELECT_STATS: &str = "
SELECT
    (SELECT COUNT(user_id) FROM pfe.users) AS total_users,
    (SELECT COUNT(company_id) FROM pfe.companies) AS total_companies,
    (SELECT COUNT(form_id) FROM pfe.forms) AS total_forms,
    (SELECT COUNT(question_id) FROM pfe.questions) AS total_questions,
    (SELECT COUNT(answer_id) FROM pfe.answers_esg) AS total_answers,
    (SELECT COUNT(tc.template_id)
     FROM pfe.template_company tc
     JOIN pfe.templates t ON tc.template_id = t.template_id
     WHERE t.value = 'ALL') AS total_templates_all,
    (SELECT COUNT(tc.template_id)
     FROM pfe.template_company tc
     JOIN pfe.templates t ON tc.template_id = t.template_id
     WHERE t.value = 'WORKERS') AS total_templates_workers,
    (SELECT COUNT(tc.template_id)
     FROM pfe.template_company tc
     JOIN pfe.templates t ON tc.template_id = t.template_id
     WHERE t.value = 'OWNED FACILITY') AS total_templates_owned_facility,
    (SELECT COUNT(tc.template_id)
     FROM pfe.template_company tc
     JOIN pfe.templates t ON tc.template_id = t.template_id
     WHERE t.value = 'PRODUITS') AS total_templates_products,
    (SELECT COUNT(tc.template_id)
     FROM pfe.template_company tc
     JOIN pfe.templates t ON tc.template_id = t.template_id
     WHERE t.value = 'FACILITY') AS total_templates_facility,
    (SELECT COUNT(onboarding_id) FROM pfe.onboarding) AS total_onboarding,
    (SELECT COUNT(CASE WHEN status = 'ACCEPTED' THEN 1 END) FROM pfe.onboarding) AS total_accepted_onboarding,
    (SELECT COUNT(CASE WHEN status = 'REJECTED' THEN 1 END) FROM pfe.onboarding) AS total_rejected_onboarding;
";

#[derive(Debug, Clone)]
pub struct StatsService {
    pub db: Pool<Postgres>,
}

impl StatsService {
    pub async fn get_stats(&self) -> Result<Stats, ResponseError> {
        let stats = sqlx::query_as::<_, Stats>(QUERY_SELECT_STATS)
            .fetch_one(&self.db)
            .await
            .map_err(|e| {
                tracing::error!("Failed to fetch stats: {:?}", e);
                ResponseError::DbError(e)
            })?;

        Ok(stats)
    }
}