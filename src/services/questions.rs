use sqlx::{Pool, Postgres, Row};
use crate::{errors::questionserror::QuestionError, routes::questions::QuestionRequest};

#[derive(Debug, Clone)]
pub struct QuestionService {
    pub db: Pool<Postgres>,
}

impl QuestionService {
    pub async fn create_question(
        &self,
        question: QuestionRequest,
    ) -> Result<i32, QuestionError> {
        let query = "
            INSERT INTO pfe.questions (question, question_status, category, sub_category)
            VALUES ($1, $2, $3, $4)
            RETURNING id
        ";
        let question_id: i32 = sqlx::query(query)
            .bind(&question.question)
            .bind(&question.question_status)
            .bind(&question.category)
            .bind(&question.sub_category)
            .fetch_one(&self.db)
            .await.map_err(QuestionError::DbError)?
            .get("id");
    
        Ok(question_id)
    }
}