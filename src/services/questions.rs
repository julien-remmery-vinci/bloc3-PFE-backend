use sqlx::{Pool, Postgres};
use crate::{
    errors::responserror::ResponseError, 
    models::question::{PutQuestionRequest, Question, QuestionForm, QuestionRequest},
};

const READ_BY_ID_QUERY: &str = "
        SELECT question_id, category, sub_category, question, is_used, question_type
        FROM pfe.questions
        WHERE question_id = $1
    ";
const READ_ALL_BY_TYPE_QUERY: &str = "
        SELECT question_id, category, sub_category, question, is_used, question_type
        FROM pfe.questions
        WHERE question_type = $1
    ";
const READ_ALL_USED_QUERY: &str = "
        SELECT question_id, category, sub_category, question, is_used, question_type
        FROM pfe.questions
        WHERE is_used = true AND question_type = $1
    ";
const READ_ALL_BY_FORM_ID_QUERY: &str = "
        SELECT q.question_id AS question_id, q.category, q.sub_category, q.question, q.is_used, qf.question_status, q.question_type
        FROM pfe.questions_form qf
        JOIN pfe.questions q ON qf.question_id = q.question_id
        WHERE qf.form_id = $1
    ";
const INSERT_QUERY: &str ="
        INSERT INTO pfe.questions (question, category, sub_category, question_type, is_used)
        VALUES ($1, $2, $3, $4, $5)
    ";
const UPDATE_QUESTION_QUERY: &str = "
        UPDATE pfe.questions
        SET question = $1
        WHERE question_id = $2
    ";
const UPDATE_QUESTION_USAGE_QUERY: &str = "
        UPDATE pfe.questions
        SET is_used = $1
        WHERE question_id = $2
    ";

#[derive(Debug, Clone)]
pub struct QuestionService {
    pub db: Pool<Postgres>,
}

impl QuestionService {
    pub async fn create_question(
        &self,
        question: QuestionRequest,
    ) -> Result<(), ResponseError> {
        sqlx::query::<_>(INSERT_QUERY)
            .bind(&question.question)
            .bind(&question.category)
            .bind(&question.sub_category)
            .bind(&question.question_type)
            .bind(&question.is_used)
            .execute(&self.db)
            .await.map_err(ResponseError::DbError)?;
        Ok(())
    }

    pub async fn read_one_question(
        &self,
        id: i32,
    ) -> Result<Question, ResponseError> {
        match sqlx::query_as::<_, Question>(READ_BY_ID_QUERY)
            .bind(id)
            .fetch_optional(&self.db)
            .await.map_err(ResponseError::DbError) {
                Ok(Some(question)) => Ok(question),
                Ok(None) => Err(ResponseError::NotFound(Some(String::from("Question not found")))),
                Err(err) => Err(err),
            }
    }

    pub async fn update_question(
        &self,
        id: i32,
        question: PutQuestionRequest,
    ) -> Result<(), ResponseError> {
        if let Some(is_used) = question.is_used {
            sqlx::query(UPDATE_QUESTION_USAGE_QUERY)
                .bind(&is_used)
                .bind(id)
                .execute(&self.db)
                .await.map_err(ResponseError::DbError)?;
            return Ok(());
        }
        if let Some(question) = question.question {
        sqlx::query(UPDATE_QUESTION_QUERY)
            .bind(&question)
            .bind(id)
            .execute(&self.db)
            .await.map_err(ResponseError::DbError)?;
        }
    
        Ok(())
    }

    pub async fn read_all_questions_by_type(
        &self,
        question_type: String
    ) -> Result<Vec<Question>, ResponseError> {
        let questions = sqlx::query_as::<_, Question>(READ_ALL_BY_TYPE_QUERY)
            .bind(question_type)
            .fetch_all(&self.db)
            .await.map_err(ResponseError::DbError)?;
        Ok(questions)
    }

    pub async fn read_all_used_questions(
        &self,
        question_type: String
    ) -> Result<Vec<Question>, ResponseError> {
        let questions = sqlx::query_as::<_, Question>(READ_ALL_USED_QUERY)
            .bind(question_type)
            .fetch_all(&self.db)
            .await.map_err(ResponseError::DbError)?;

        Ok(questions)
    }

    pub async fn read_all_by_form_id(
        &self,
        form_id: i32,
    ) -> Result<Vec<Question>, ResponseError> {
        let questions = sqlx::query_as::<_, Question>(READ_ALL_BY_FORM_ID_QUERY)
            .bind(form_id)
            .fetch_all(&self.db)
            .await.map_err(ResponseError::DbError)?;
        Ok(questions)
    }

    pub async fn complete_question(
        &self,
        question_id: i32,
        form_id: i32,
    ) -> Result<(), ResponseError> {
        let query = "
            UPDATE pfe.questions_form
            SET question_status = 'COMPLETE'
            WHERE question_id = $1 AND form_id = $2
        ";
        sqlx::query(query)
            .bind(question_id)
            .bind(form_id)
            .execute(&self.db)
            .await.map_err(ResponseError::DbError)?;
        Ok(())
    }

    pub async fn read_all_questions_forms_by_form_id(
        &self,
        form_id: i32,
    ) -> Result<Vec<QuestionForm>, ResponseError> {
        let query = "
            SELECT qf.form_id, qf.question_id, qf.question_status
            FROM pfe.questions_form qf
            WHERE qf.form_id = $1
        ";
        let questions = sqlx::query_as::<_, QuestionForm>(query)
            .bind(form_id)
            .fetch_all(&self.db)
            .await.map_err(ResponseError::DbError)?;
        Ok(questions)
    }
}