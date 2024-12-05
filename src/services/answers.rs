use crate::{
    models::answers::Answer,
    errors::answer_error::AnswerError,
    routes::answers::CreateAnswer,
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