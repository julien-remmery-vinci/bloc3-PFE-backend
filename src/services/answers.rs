use crate::{
    errors::globalerror::ResponseError,
    models::answer::{Answer, AnswerUser, CreateAnswer, CreateAnswerUser},
};

#[derive(Debug, Clone)]
pub struct AnswerService {
    pub db: sqlx::PgPool,
}

const QUERY_INSERT_ANSWER: &str = "
    INSERT INTO pfe.answers_esg (question_id, template, answer, score_now, score_commitment_pact, is_forced_engagement) 
    VALUES ($1, $2, $3, $4, $5, $6) 
    RETURNING answer_id, question_id, template, answer, score_now, score_commitment_pact, is_forced_engagement
";

const QUERY_INSERT_ANSWER_USER: &str = "
    INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, answer, now, commitment_pact, comment)
    VALUES ($1, $2, $3, $4, $5, $6, $7)
    RETURNING answer_id, user_id, form_id, answer, now, commitment_pact, comment, now_verif, commitment_pact_verif
";

const QUERY_FIND_ANSWER_USER_BY_FORM_ID: &str = "
    SELECT answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif
    FROM pfe.user_answer_esg
    WHERE form_id = $1 AND user_id = $2 AND answer_id = $3
    ";

const QUERY_READ_ANSWERS_BY_USER_BY_QUESTION: &str = "
    SELECT answer_id, user_id, form_id, answer, now, commitment_pact, comment, now_verif, commitment_pact_verif
    FROM pfe.user_answer_esg
    WHERE user_id = $1 AND form_id = $3 AND answer_id IN (SELECT answer_id FROM pfe.answers_esg WHERE question_id = $2)
    ";


impl AnswerService {
    pub async fn create_answer(&self, answer: CreateAnswer) -> Result<Answer, ResponseError> {
        match sqlx::query_as::<_, Answer>(QUERY_INSERT_ANSWER)
            .bind(answer.question_id.clone())
            .bind(answer.template.clone())
            .bind(answer.answer.clone())
            .bind(answer.score.clone())
            .bind(answer.engagement_score.clone())
            .bind(answer.is_forced_engagement.clone())
            .fetch_one(&self.db)
            .await
            .map_err(|error| ResponseError::DbError(error))
        {
            Ok(answer) => Ok(answer),
            Err(error) => Err(error),
        }
    }

    pub async fn create_answer_user(&self, answer: CreateAnswerUser, user_id: i32, answer_id: i32) -> Result<AnswerUser, ResponseError> {
        match sqlx::query_as::<_, AnswerUser>(QUERY_INSERT_ANSWER_USER)
            .bind(answer_id.clone())
            .bind(user_id.clone())
            .bind(answer.form_id.clone())
            .bind(answer.answer.clone())
            .bind(answer.now.clone())
            .bind(answer.commitment_pact.clone())
            .bind(answer.comment.clone())
            .fetch_one(&self.db)
            .await
            .map_err(|error| ResponseError::DbError(error))
        {
            Ok(answer) => Ok(answer),
            Err(error) => Err(error),
        }
    }

    pub async fn read_answer_by_id(&self, answer_id: i32) -> Result<Option<Answer>, ResponseError> {
        match sqlx::query_as::<_, Answer>("SELECT * FROM pfe.answers_esg WHERE answer_id = $1")
            .bind(answer_id)
            .fetch_optional(&self.db)
            .await
            .map_err(|error| ResponseError::DbError(error))
        {
            Ok(answer) => Ok(answer),
            Err(error) => Err(error),
        }
    }

    pub async fn read_answer_user_by_form_id(&self, form_id: i32, user_id: i32,answer_id: i32) -> Result<Option<AnswerUser>, ResponseError> {
        match sqlx::query_as::<_, AnswerUser>(QUERY_FIND_ANSWER_USER_BY_FORM_ID)
            .bind(form_id)
            .bind(user_id)
            .bind(answer_id)
            .fetch_optional(&self.db)
            .await
            .map_err(|error| ResponseError::DbError(error))
        {
            Ok(answer) => Ok(answer),
            Err(error) => Err(error),
        }
    }

    pub async fn read_answers_by_question(&self, question_id: i32) -> Result<Vec<Answer>, ResponseError> {
        match sqlx::query_as::<_, Answer>("SELECT * FROM pfe.answers_esg WHERE question_id = $1")
            .bind(question_id)
            .fetch_all(&self.db)
            .await
            .map_err(|error| ResponseError::DbError(error))
        {
            Ok(answers) => Ok(answers),
            Err(error) => Err(error),
        }
    }

    pub async fn read_possible_answer_by_id(&self, answer_id: i32) -> Result<Option<Answer>, ResponseError> {
        match sqlx::query_as::<_, Answer>("SELECT * FROM pfe.answers_esg WHERE answer_id = $1 AND answer IS NULL")
            .bind(answer_id)
            .fetch_optional(&self.db)
            .await
            .map_err(|error| ResponseError::DbError(error))
        {
            Ok(answer) => Ok(answer),
            Err(error) => Err(error),
        }   
    }

    // method to retrieve all answers of a user for a specific question
    pub async fn read_answers_by_user_by_question(&self, user_id: i32, question_id: i32, form_id: i32) -> Result<Vec<AnswerUser>, ResponseError> {
        match sqlx::query_as::<_, AnswerUser>(QUERY_READ_ANSWERS_BY_USER_BY_QUESTION)
            .bind(user_id)
            .bind(question_id)
            .bind(form_id)
            .fetch_all(&self.db)
            .await
            .map_err(|error| ResponseError::DbError(error))
        {
            Ok(answers) => Ok(answers),
            Err(error) => Err(error),
        }
    }

}