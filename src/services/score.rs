use crate::{errors::responserror::ResponseError, models::score::ScoreQuery};

#[derive(Debug,Clone)]
pub struct ScoreService {
    pub db: sqlx::PgPool,
}

/*
const QUERY_SUM_SCORE_TEMPLATE: &str = "
    SELECT SUM(score_now) as score
    FROM pfe.answers_esg
    WHERE template = $1
    ";

const QUERY_SUM_SCORE_TEMPLATE_ALL: &str = "
    SELECT SUM(score_now) as score
    FROM pfe.answers_esg
    ";

const QUERY_FIND_TEMPLATE_BY_FORM_ID: &str = "
    SELECT value
    FROM pfe.templates
    WHERE template_id IN (SELECT template_id FROM pfe.template_form WHERE form_id = $1)
    ";*/

const QUERY_SUM_SCORE_USER_NOW: &str = "
    SELECT q.sub_category, SUM(a.score_now) as score
    FROM pfe.answers_esg a
    JOIN pfe.user_answer_esg u ON a.answer_id = u.answer_id
    JOIN pfe.questions q ON a.question_id = q.question_id
    WHERE u.form_id = $1 AND u.now_verif = TRUE
    GROUP BY q.sub_category
    ";

const QUERY_SUM_SCORE_USER_COMMITMENT_PACT: &str = "
    SELECT q.sub_category, SUM(a.score_commitment_pact) as score
    FROM pfe.answers_esg a
    JOIN pfe.user_answer_esg u ON a.answer_id = u.answer_id
    JOIN pfe.questions q ON a.question_id = q.question_id
    WHERE u.form_id = $1 AND u.commitment_pact_verif = TRUE
    GROUP BY q.sub_category
    ";


impl ScoreService {
    //FIRST STEP
    //returns the sum of the scores of all the answers that have the same template
    /* 
    pub async fn sum_score_template(&self, template: String) -> Result<f64, ResponseError> {
        let score: (f64,) = sqlx::query_as(QUERY_SUM_SCORE_TEMPLATE)
            .bind(template)
            .fetch_one(&self.db)
            .await
            .map_err(ResponseError::DbError)?;
        Ok(score.0)
    }

    pub async fn sum_score_template_all(&self) -> Result<f64, ResponseError> {
        let score: (f64,) = sqlx::query_as(QUERY_SUM_SCORE_TEMPLATE_ALL)
            .bind("ALL")
            .fetch_one(&self.db)
            .await
            .map_err(ResponseError::DbError)?;
        Ok(score.0)
    }
    //but first we need to find the template(s) ( 1 or many templates per form) of the form knowing the form_id
    pub async fn find_template_by_form_id(&self, form_id: i32) -> Result<Vec<String>, ResponseError> {
        let templates: Vec<(Option<String>,)> = sqlx::query_as(QUERY_FIND_TEMPLATE_BY_FORM_ID)
            .bind(form_id)
            .fetch_all(&self.db)
            .await
            .map_err(ResponseError::DbError)?;
        
        Ok(templates
            .into_iter()
            .filter_map(|(template,)| template)
            .collect())
    }
    */

    //SECOND STEP
    // now we need to sum the score that the user actually choose (now and commitment_pact)
    pub async fn sum_score_user_now(&self, form_id: i32) -> Result<Option<Vec<ScoreQuery>>, ResponseError> {
        let scores: Vec<ScoreQuery> = sqlx::query_as(QUERY_SUM_SCORE_USER_NOW)
            .bind(form_id)
            .fetch_all(&self.db)
            .await
            .map_err(ResponseError::DbError)?;
        
        Ok(Some(scores))
    }

    pub async fn sum_score_user_commitment_pact(&self, form_id: i32) -> Result<Option<Vec<ScoreQuery>>, ResponseError> {
        let scores: Vec<ScoreQuery> = sqlx::query_as(QUERY_SUM_SCORE_USER_COMMITMENT_PACT)
            .bind(form_id)
            .fetch_all(&self.db)
            .await
            .map_err(ResponseError::DbError)?;
        
        Ok(Some(scores))
    }
}