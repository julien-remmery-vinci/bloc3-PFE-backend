use crate::errors::responserror::ResponseError;

#[derive(Debug,Clone)]
pub struct ScoreService {
    pub db: sqlx::PgPool,
}

const QUERY_SUM_SCORE_TEMPLATE: &str = "
    SELECT SUM(score_now) as score
    FROM pfe.answers_esg
    WHERE template = $1
    ";

const QUERY_FIND_TEMPLATE_BY_FORM_ID: &str = "
    SELECT value
    FROM pfe.templates
    WHERE template_id = (SELECT template_id FROM pfe.template_form WHERE form_id = $1)
    ";

impl ScoreService {
    //FIRST STEP
    //returns the sum of the scores of all the answers that have the same template
    pub async fn sum_score_template(&self, template: String) -> Result<f64, ResponseError> {
        let score: (f64,) = sqlx::query_as(QUERY_SUM_SCORE_TEMPLATE)
            .bind(template)
            .fetch_one(&self.db)
            .await
            .map_err(ResponseError::DbError)?;
        Ok(score.0)
    }
    //but first we need to find the template of the form knowing the form_id
    pub async fn find_template_by_form_id(&self, form_id: i32) -> Result<String, ResponseError> {
        let template: (String,) = sqlx::query_as(QUERY_FIND_TEMPLATE_BY_FORM_ID)
            .bind(form_id)
            .fetch_one(&self.db)
            .await
            .map_err(ResponseError::DbError)?;
        Ok(template.0)
    }
}