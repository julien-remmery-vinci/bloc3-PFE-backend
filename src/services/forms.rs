use crate::models::form::Form;
use sqlx::PgPool;

const QUERY_INSERT_FORM: &str = "
    INSERT INTO pfe.forms (company_id, type, nb_questions, template)
    VALUES ($1, $2, $3, $4)
    RETURNING form_id, company_id, type AS form_type, nb_questions, template
";

pub async fn create_form_in_db(db: &PgPool, new_form: Form) -> Result<Form, sqlx::error::Error> {
    match sqlx::query_as::<_, Form>(QUERY_INSERT_FORM)
        .bind(new_form.company_id)
        .bind(new_form.form_type)
        .bind(new_form.nb_questions)
        .bind(new_form.template)
        .fetch_one(db)
        .await
    {
        Ok(form) => Ok(form),
        Err(error) => Err(error),
    }   
    
}