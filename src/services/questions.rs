use sqlx::{Pool, Postgres};
use crate::{errors::questionserror::QuestionError, routes::questions::{PutQuestionRequest, QuestionRequest}};

const READ_BY_ID_QUERY: &str = "
            SELECT question, category, sub_category
            FROM pfe.questions
            WHERE id = $1
        ";
const READ_ALL_QUERY: &str = "
            SELECT question, category, sub_category
            FROM pfe.questions
        ";
const INSERT_QUERY: &str ="
            INSERT INTO pfe.questions (question, category, sub_category)
            VALUES ($1, $2, $3)
            RETURNING id
        ";
const UPDATE_QUERY: &str = "
            UPDATE pfe.questions
            SET question = $1
            WHERE id = $2
        ";

#[derive(Debug, Clone)]
pub struct QuestionService {
    pub db: Pool<Postgres>,
}

impl QuestionService {
    pub async fn create_question(
        &self,
        question: QuestionRequest,
    ) -> Result<i32, QuestionError> {
        let question_id: i32 = sqlx::query_scalar(INSERT_QUERY)
            .bind(&question.question)
            .bind(&question.category)
            .bind(&question.sub_category)
            .fetch_one(&self.db)
            .await.map_err(QuestionError::DbError)?;
        Ok(question_id)
    }

    pub async fn read_one_question(
        &self,
        id: i32,
    ) -> Result<QuestionRequest, QuestionError> {
        match sqlx::query_as::<_, QuestionRequest>(READ_BY_ID_QUERY)
            .bind(id)
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
        sqlx::query(UPDATE_QUERY)
            .bind(&question.question)
            .bind(id)
            .execute(&self.db)
            .await.map_err(QuestionError::DbError)?;
    
        Ok(())
    }

    pub async fn read_all_questions(
        &self,
    ) -> Result<Vec<QuestionRequest>, QuestionError> {
        let questions = sqlx::query_as::<_, QuestionRequest>(READ_ALL_QUERY)
            .fetch_all(&self.db)
            .await.map_err(QuestionError::DbError)?;
    
        Ok(questions)
    }
}