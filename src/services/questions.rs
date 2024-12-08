use sqlx::{Pool, Postgres};
use crate::{errors::{globalerror::GlobalError, questionserror::QuestionError}, models::{question::Question}, routes::questions::{PutQuestionRequest, QuestionRequest}};

const READ_BY_ID_QUERY: &str = "
        SELECT question_id, category, sub_category
        FROM pfe.questions
        WHERE id = $1
    ";
const READ_ALL_QUERY: &str = "
        SELECT question_id, category, sub_category
        FROM pfe.questions
    ";
const READ_ALL_USED_QUERY: &str = "
        SELECT question_id, category, sub_category, question, is_used
        FROM pfe.questions
        WHERE is_used = true AND question_type = $1
    ";
const READ_ALL_BY_FORM_ID_QUERY: &str = "
        SELECT q.question_id AS question_id, q.category, q.sub_category, q.question, q.is_used, qf.question_status
        FROM pfe.questions_form qf
        JOIN pfe.questions q ON qf.question_id = q.question_id
        WHERE qf.form_id = $1
    ";
const INSERT_QUERY: &str ="
        INSERT INTO pfe.questions (question_id, category, sub_category)
        VALUES ($1, $2, $3)
        RETURNING id
    ";
const UPDATE_QUERY: &str = "
        UPDATE pfe.question_id
        SET question = $1
        WHERE id = $2
    ";
const DELETE_QUERY: &str = "
        UPDATE pfe.question_id
        SET is_used = $1
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
        if let Some(is_used) = question.is_used {
            sqlx::query(DELETE_QUERY)
                .bind(&is_used)
                .bind(id)
                .execute(&self.db)
                .await.map_err(QuestionError::DbError)?;
            return Ok(());
        }
        if let Some(question) = question.question {
        sqlx::query(UPDATE_QUERY)
            .bind(&question)
            .bind(id)
            .execute(&self.db)
            .await.map_err(QuestionError::DbError)?;
        }
    
        Ok(())
    }

    pub async fn read_all_questions(
        &self
    ) -> Result<Vec<QuestionRequest>, QuestionError> {
        let questions = sqlx::query_as::<_, QuestionRequest>(READ_ALL_QUERY)
            .fetch_all(&self.db)
            .await.map_err(QuestionError::DbError)?;
        Ok(questions)
    }

    pub async fn read_all_used_questions(
        &self,
        question_type: String
    ) -> Result<Vec<Question>, GlobalError> {
        let questions = sqlx::query_as::<_, Question>(READ_ALL_USED_QUERY)
            .bind(question_type)
            .fetch_all(&self.db)
            .await.map_err(GlobalError::DbError)?;

        Ok(questions)
    }

    pub async fn read_all_by_form_id(
        &self,
        form_id: i32,
    ) -> Result<Vec<Question>, GlobalError> {
        let questions = sqlx::query_as::<_, Question>(READ_ALL_BY_FORM_ID_QUERY)
            .bind(form_id)
            .fetch_all(&self.db)
            .await.map_err(GlobalError::DbError)?;
        Ok(questions)
    }

    pub async fn delete_question(
        &self,
        id: i32,
    ) -> Result<(), QuestionError> {
        sqlx::query(DELETE_QUERY)
            .bind(id)
            .execute(&self.db)
            .await.map_err(QuestionError::DbError)?;
    
        Ok(())
    }
}