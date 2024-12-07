use crate::errors::score_error::ScoreError;

#[derive(Debug,Clone)]
pub struct ScoreService {
    pub db: sqlx::PgPool,
}

const QUERY_SUM_SCORE_TEMPLATE: &str = "
    SELECT SUM(score) as score
    FROM pfe.answers
    WHERE template = $1
    ";

const QUERY_FIND_COMPANY_ID_BY_FORM_ID: &str = "
    SELECT company
    FROM pfe.forms
    WHERE id = $1
    ";

//returns the sum of the scores of all the answers that have the same template
impl ScoreService {
    pub async fn sum_score_template(&self, template: String) -> Result<f64, ScoreError> {
        sqlx::query_scalar::<_, f64>(QUERY_SUM_SCORE_TEMPLATE)
            .bind(template)
            .fetch_one(&self.db)
            .await
            .map_err(ScoreError::DbError)
    }

    pub async fn find_company_by_form_id(&self, form_id: i32) -> Result<String, ScoreError> {
        let company = sqlx::query_scalar::<_, String>(QUERY_FIND_COMPANY_ID_BY_FORM_ID)
            .bind(form_id)
            .fetch_one(&self.db)
            .await
            .map_err(ScoreError::DbError)?;

        Ok(company)
    }

    pub async fn find_template_by_form_id(&self, form_id: i32) -> Result<String, ScoreError> {
        let template = sqlx::query_scalar::<_, String>(,
        )
        .bind(form_id)
        .fetch_one(&self.db)
        .await
        .map_err(ScoreError::DbError)?;

        Ok(template)
    }
}