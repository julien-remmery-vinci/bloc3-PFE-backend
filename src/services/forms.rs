use crate::errors::responserror::ResponseError;
use crate::models::form::{CreateForm, Form};
use crate::models::question::Question;
use crate::models::template::Template;

use sqlx::{Error, PgPool};

const QUERY_INSERT_FORM: &str = "
    INSERT INTO pfe.forms (company_id, type)
    VALUES ($1, $2)
    RETURNING form_id, company_id, type
";
// const QUERY_SELECT_FORM: &str = "
//     SELECT form_id, company_id, type AS form_type
//     FROM pfe.forms
//     WHERE form_id = $1
// ";
const QUERY_SELECT_FORMS_BY_COMPANY: &str = "
    SELECT form_id, company_id, type
    FROM pfe.forms
    WHERE company_id = $1
";
const QUERY_INSERT_QUESTION_FORM: &str = "
    INSERT INTO pfe.questions_form (form_id, question_id, question_status)
    VALUES ($1, $2, $3)
    RETURNING form_id, question_id, question_status
";

const QUERY_INSERT_TEMPLATE_FORM: &str = "
    INSERT INTO pfe.template_form (form_id, template_id)
    VALUES ($1, $2)
";

const QUERY_SELECT_TEMPLATES_BY_FORM: &str = "
    SELECT t.template_id, value
    FROM pfe.templates t, pfe.template_form tf
    WHERE t.template_id = tf.template_id AND tf.form_id = $1
";

#[derive(Debug, Clone)]
pub struct FormService {
    pub db: PgPool,
}

impl FormService {
    // Inserer un formulaire et ses questions dans la base de données
    pub async fn create_form_in_db(&self, new_form: CreateForm, questions: Vec<Question>, templates: Vec<Template>) -> Result<(), Error> {
        let form = sqlx::query_as::<_, Form>(QUERY_INSERT_FORM)
            .bind(new_form.company_id)
            .bind(new_form.r#type)
            .fetch_one(&self.db)
            .await?;

        for question in questions {
            sqlx::query(QUERY_INSERT_QUESTION_FORM)
                .bind(form.form_id)
                .bind(question.question_id)
                .bind(String::from("PENDING"))
                .execute(&self.db)
                .await?;
        }

        for template in templates {
            sqlx::query(QUERY_INSERT_TEMPLATE_FORM)
                .bind(form.form_id)
                .bind(template.template_id)
                .execute(&self.db)
                .await?;
        }
        Ok(())
    }

    // Lire les formulaires d'une entreprise
    pub async fn read_forms_by_company(&self, company_id: i32) -> Result<Vec<Form>, ResponseError> {
        let forms = sqlx::query_as::<_, Form>(QUERY_SELECT_FORMS_BY_COMPANY)
            .bind(company_id)
            .fetch_all(&self.db)
            .await.map_err(|e| ResponseError::DbError(e))?;
        Ok(forms)
    }

    pub async fn read_form_templates(&self, form_id: i32) -> Result<Vec<Template>, ResponseError> {
        let templates = sqlx::query_as::<_, Template>(QUERY_SELECT_TEMPLATES_BY_FORM)
            .bind(form_id)
            .fetch_all(&self.db)
            .await.map_err(ResponseError::DbError)?;
        Ok(templates)
    }
    
    // Lire un formulaire par ID
    // pub async fn read_form_in_db(&self, form_id: i32) -> Result<Form, Error> {
    //     let form = sqlx::query_as::<_, Form>(QUERY_SELECT_FORM)
    //         .bind(form_id)
    //         .fetch_one(&self.db)
    //         .await?;
    
    //     let questions = sqlx::query_as::<_, Question>(QUERY_SELECT_QUESTIONS_BY_FORM)
    //         .bind(form_id)
    //         .fetch_all(&self.db)
    //         .await?;
    
    //     let mut questions_with_answers = Vec::new();
    //     for question in questions {
    //         let answers = sqlx::query_as::<_, Answer>(QUERY_SELECT_ANSWERS_BY_QUESTION)
    //             .bind(question.id)
    //             .fetch_all(&self.db)
    //             .await?;
    
    //         questions_with_answers.push(QuestionWithAnswers { question, answers });
    //     }
    
    //     Ok(Form {
    //         questions: Some(serde_json::to_value(questions_with_answers)
    //             .map_err(|e| sqlx::Error::Decode(Box::new(e)))?),
    //         ..form
    //     })
        
    // }
}
