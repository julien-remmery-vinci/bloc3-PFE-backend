use crate::{
    errors::responserror::ResponseError,
    models::answer::{Answer, AnswerUser, CreateAnswer, CreateAnswerUser, CreateAnswerValidation},
};

#[derive(Debug, Clone)]
pub struct AnswerService {
    pub db: sqlx::PgPool,
}

const QUERY_INSERT_ANSWER: &str = "
    INSERT INTO pfe.answers_esg (question_id, template, answer, score_now, score_commitment_pact, is_forced_engagement, is_forced_comment) 
    VALUES ($1, $2, $3, $4, $5, $6, $7) 
    RETURNING answer_id, question_id, template, answer, score_now, score_commitment_pact, is_forced_engagement, is_forced_comment
";

const QUERY_INSERT_ANSWER_USER: &str = "
    INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment, status)
    VALUES ($1, $2, $3, $4, $5, $6, 'PENDING')
    RETURNING answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status
";

const QUERY_FIND_ANSWER_USER_BY_FORM_ID: &str = "
    SELECT answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status
    FROM pfe.user_answer_esg
    WHERE form_id = $1 AND answer_id = $2
";

const QUERY_READ_ALL_ANSWERS_BY_QUESTION: &str = "
    SELECT answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status
    FROM pfe.user_answer_esg
    WHERE form_id = $2 AND answer_id IN (SELECT answer_id FROM pfe.answers_esg WHERE question_id = $1)
";

const QUERY_READ_ANSWER_BY_QUESTION: &str = "
    SELECT answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status
    FROM pfe.user_answer_esg
    WHERE form_id = $3 AND answer_id = $1
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
            .bind(answer.is_forced_comment.clone())
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

    pub async fn read_answer_by_form_id(&self, form_id: i32, answer_id: i32) -> Result<Option<AnswerUser>, ResponseError> {
        match sqlx::query_as::<_, AnswerUser>(QUERY_FIND_ANSWER_USER_BY_FORM_ID)
            .bind(form_id)
            .bind(answer_id)
            .fetch_optional(&self.db)
            .await
            .map_err(|error| ResponseError::DbError(error))
        {
            Ok(answer) => Ok(answer),
            Err(error) => Err(error),
        }
    }

    pub async fn delete_user_answer_by_form_id(&self, form_id: i32, answer_id: i32) -> Result<(), ResponseError> {
        match sqlx::query("DELETE FROM pfe.user_answer_esg WHERE form_id = $1 AND answer_id = $2")
            .bind(form_id)
            .bind(answer_id)
            .execute(&self.db)
            .await
            .map_err(|error| ResponseError::DbError(error))
        {
            Ok(_) => Ok(()),
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

    // method to retrieve all answers of a user for a specific question
    pub async fn read_answers_by_user_by_question(&self, question_id: i32, form_id: i32) -> Result<Vec<AnswerUser>, ResponseError> {
        match sqlx::query_as::<_, AnswerUser>(QUERY_READ_ALL_ANSWERS_BY_QUESTION)
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

    pub async fn read_user_answer_by_question(&self, answer_id: i32, question_id: i32, form_id: i32) -> Result<Option<AnswerUser>, ResponseError> {
        match sqlx::query_as::<_, AnswerUser>(QUERY_READ_ANSWER_BY_QUESTION)
            .bind(answer_id)
            .bind(question_id)
            .bind(form_id)
            .fetch_optional(&self.db)
            .await
            // .map_err(|error| ResponseError::DbError(error))
        {
            Ok(answers) => Ok(answers),
            Err(error) =>  {
                tracing::error!("Error reading user answer by question: {:?}", error);
                Err(ResponseError::DbError(error))
            },
        }
    }

    pub async fn validate_user_answer(&self, validated: AnswerUser) -> Result<AnswerUser, ResponseError> {
        match sqlx::query_as::<_, AnswerUser>("
                UPDATE pfe.user_answer_esg 
                SET now_verif = $3, commitment_pact_verif = $4, comment = $5, status = 'VALIDATED'
                WHERE answer_id = $1 AND form_id = $2
                RETURNING answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status
            ")
            .bind(validated.answer_id)
            .bind(validated.form_id)
            .bind(validated.now_verif.clone())
            .bind(validated.commitment_pact_verif.clone())
            .bind(validated.comment.clone())
            .fetch_one(&self.db)
            .await 
        {
            Ok(answer) => Ok(answer),
            Err(error) => {
                tracing::error!("Error validating answer: {:?}", error);
                Err(ResponseError::DbError(error))
            },
        }
    }

    pub async fn insert_answer_validation(&self, validated: CreateAnswerValidation) -> Result<AnswerUser, ResponseError> {
        let answer = sqlx::query_as::<_, AnswerUser>("
                INSERT INTO pfe.user_answer_esg (answer_id, form_id, user_id, comment, now_verif, commitment_pact_verif, status)
                VALUES ($1, $2, $3, $4, $5, $6, 'VALIDATED')
                RETURNING answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif, status
            ")
            .bind(validated.answer_id)
            .bind(validated.form_id)
            .bind(validated.user_id)
            .bind(validated.comment.clone())
            .bind(validated.now_verif.clone())
            .bind(validated.commitment_pact_verif.clone())
            .fetch_one(&self.db)
            .await
            .map_err(ResponseError::DbError)?;
        Ok(answer)
    }

    pub async fn update_answer_score(&self, answer_id: i32, score: f64, engagement_score: f64) -> Result<Answer, ResponseError> {
        let answer = sqlx::query_as::<_, Answer>("
                UPDATE pfe.answers_esg 
                SET score_now = $2, score_commitment_pact = $3
                WHERE answer_id = $1
                RETURNING answer_id, question_id, template, answer, score_now, score_commitment_pact, is_forced_engagement, is_forced_comment
            ")
            .bind(answer_id)
            .bind(score)
            .bind(engagement_score)
            .fetch_one(&self.db)
            .await
            .map_err(ResponseError::DbError)?;
        Ok(answer)
    }
}