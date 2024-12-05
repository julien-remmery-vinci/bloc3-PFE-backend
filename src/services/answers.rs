use crate::{
    errors::answer_error::AnswerError,
    models::answers::Answer,
    models::answerusers::AnswerUser,
    routes::answers::{CreateAnswer, CreateAnswerUser}
};

#[derive(Debug, Clone)]
pub struct AnswerService {
    pub db: sqlx::PgPool,
}

const QUERY_INSERT_ANSWER: &str = "
    INSERT INTO pfe.answers (answer, template, question_id, score, engagement_score, is_forced_engagement, comment) 
    VALUES ($1, $2, $3, $4, $5, $6, $7) 
    RETURNING answer_id, answer, template, question_id, score, engagement_score, is_forced_engagement, comment
";

const QUERY_INSERT_ANSWER_USER: &str = "
    INSERT INTO pfe.user_answer_esg (answer_id, user_id, form_id, now, commitment_pact, comment)
    VALUES ($1, $2, $3, $4, $5, $6)
    RETURNING answer_id, user_id, form_id, now, commitment_pact, comment, now_verif, commitment_pact_verif
";

impl AnswerService {
    pub async fn create_answer(&self, answer: CreateAnswer) -> Result<Answer, AnswerError> {
        match sqlx::query_as::<_, Answer>(QUERY_INSERT_ANSWER)
            .bind(answer.answer.clone())
            .bind(answer.template.clone())
            .bind(answer.question_id.clone())
            .bind(answer.score.clone())
            .bind(answer.engagement_score.clone())
            .bind(answer.is_forced_engagement.clone())
            .bind(answer.comment.clone())
            .fetch_one(&self.db)
            .await
            .map_err(|error| AnswerError::DbError(error))
        {
            Ok(answer) => Ok(answer),
            Err(error) => Err(error),
        }
    }

    pub async fn create_answer_user(&self, answer: CreateAnswerUser, user_id: i32, answer_id: i32) -> Result<AnswerUser, AnswerError> {
        match sqlx::query_as::<_, AnswerUser>(QUERY_INSERT_ANSWER_USER)
            .bind(answer_id.clone())
            .bind(user_id.clone())
            .bind(answer.form_id.clone())
            .bind(answer.now.clone())
            .bind(answer.commitment_pact.clone())
            .bind(answer.comment.clone())
            .fetch_one(&self.db)
            .await
            .map_err(|error| AnswerError::DbError(error))
        {
            Ok(answer) => Ok(answer),
            Err(error) => Err(error),
        }
    }

    pub async fn read_answers_by_question(&self, question_id: i32) -> Result<Vec<Answer>, AnswerError> {
        match sqlx::query_as::<_, Answer>("SELECT * FROM pfe.answers WHERE question_id = $1")
            .bind(question_id)
            .fetch_all(&self.db)
            .await
            .map_err(|error| AnswerError::DbError(error))
        {
            Ok(answers) => Ok(answers),
            Err(error) => Err(error),
        }
    }

}

/*

const QUERY_FIND_BY_QUESTION_ID_AND_ANSWER: &str = "
            SELECT id, answer, template, question_id, score, engagement_score, is_forced_engagement, comment
            FROM pfe.answers
            WHERE question_id = $1 & answer = $2
        ";
*/

/* 

    pub async fn find_by_question_id_and_answer(db: &sqlx::PgPool, question_id: i32, answer: String) -> Result<Answer, sqlx::error::Error> {
        match sqlx::query_as::<_, Answer>(QUERY_FIND_BY_QUESTION_ID_AND_ANSWER)
            .bind(question_id)
            .bind(answer)
            .fetch_all(db)
            .await
        {
            Ok(answer) => {
                if answer.is_empty() {
                    Ok(Answer::default())
                } else {
                    Ok(answer[0].clone())
                }
            },
            Err(error) => Err(error),
        }
    }

    */