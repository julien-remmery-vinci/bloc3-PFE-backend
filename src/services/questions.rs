use sqlx::{Pool, Postgres, Row};
use crate::{errors::questionserror::QuestionError, routes::questions::{PutQuestionRequest, QuestionRequest}};

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
            INSERT INTO pfe.questions (question, category, sub_category)
            VALUES ($1, $2, $3)
            RETURNING id
        ";
        let question_id: i32 = sqlx::query(query)
            .bind(&question.question)
            .bind(&question.category)
            .bind(&question.sub_category)
            .fetch_one(&self.db)
            .await.map_err(QuestionError::DbError)?
            .get("id");
    
        Ok(question_id)
    }

    pub async fn read_one_question(
        &self,
        id: i32,
    ) -> Result<QuestionRequest, QuestionError> {
        match sqlx::query_as!(QuestionRequest, "
            SELECT question, category, sub_category
            FROM pfe.questions
            WHERE id = $1
        ", id)
            .fetch_optional(&self.db)
            .await.map_err(QuestionError::DbError) {
            Ok(Some(question)) => Ok(question),
            Ok(None) => Err(QuestionError::NoSuchQuestion),
            Err(err) => Err(err),
            }
    }

    pub async fn update_question(
        &self,
        id: i32,
        question: PutQuestionRequest,
    ) -> Result<(), QuestionError> {
        let query = "
            UPDATE pfe.questions
            SET question = $1
            WHERE id = $2
        ";
        sqlx::query(query)
            .bind(&question.question)
            .bind(id)
            .execute(&self.db)
            .await.map_err(QuestionError::DbError)?;
    
        Ok(())
    }

    pub async fn get_all_questions(
        &self,
    ) -> Result<Vec<QuestionRequest>, QuestionError> {
        let questions = sqlx::query_as!(QuestionRequest, "
            SELECT question, category, sub_category
            FROM pfe.questions
        ")
            .fetch_all(&self.db)
            .await.map_err(QuestionError::DbError)?;
    
        Ok(questions)
    }
}