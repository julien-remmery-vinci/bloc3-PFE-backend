use crate::models::{
    answer::Answer,
    create_answer::CreateAnswer,
};
use crate::errors::answer_error::AnswerError;

#[derive(Debug, Clone)]
pub struct AnswerService {
    pub db: sqlx::PgPool,
}

impl AnswerService {
    pub async fn add_answer(&self, answer: CreateAnswer) -> Result<Answer, AnswerError> {
        if answer.invalid() {
            return Err(AnswerError::BadRequest);
        }
        let found_answer = Answer::find_by_question_id_and_answer(&self.db, answer.question_id.clone(), answer.answer.clone()).await
            .map_err(AnswerError::DbError)?;
        if found_answer.id != 0 {
            return Err(AnswerError::Conflict);
        }
        let created = Answer::create_answer(&self.db, answer).await.map_err(AnswerError::DbError)?;
        Ok(created)
    }

}
