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

//returns the sum of the scores of all the answers that have the same template
impl ScoreService {
    pub async fn sum_score_template(&self, template: String) -> Result<f64, ScoreError> {
        sqlx::query_scalar::<_, f64>(QUERY_SUM_SCORE_TEMPLATE)
            .bind(template)
            .fetch_one(&self.db)
            .await
            .map_err(ScoreError::DbError)
    }
}