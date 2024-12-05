use crate::models::form::Form;
use sqlx::PgPool;

const QUERY_INSERT_FORM: &str = "
    INSERT INTO pfe.forms (company_id, type, nb_questions, template)
    VALUES ($1, $2, $3, $4)
    RETURNING form_id, company_id, type AS form_type, nb_questions, template
";

const QUERY_SELECT_FORM: &str = "
    SELECT form_id, company_id, type AS form_type, nb_questions, template
    FROM pfe.forms
    WHERE form_id = $1
";

const QUERY_UPDATE_FORM: &str = "
    UPDATE pfe.forms
    SET company_id = $1, type = $2, nb_questions = $3, template = $4
    WHERE form_id = $5
    RETURNING form_id, company_id, type AS form_type, nb_questions, template
";

const QUERY_DELETE_FORM: &str = "
    DELETE FROM pfe.forms
    WHERE form_id = $1
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

pub async fn read_form_in_db(db: &PgPool, form_id: i32) -> Result<Form, sqlx::error::Error> {
    match sqlx::query_as::<_, Form>(QUERY_SELECT_FORM)
        .bind(form_id)
        .fetch_one(db)
        .await
    {
        Ok(form) => Ok(form),
        Err(error) => Err(error),
    }
}

pub async fn update_form_in_db(db: &PgPool, form_id: i32, updated_form: Form) -> Result<Form, sqlx::error::Error> {
    match sqlx::query_as::<_, Form>(QUERY_UPDATE_FORM)
        .bind(updated_form.company_id)
        .bind(updated_form.form_type)
        .bind(updated_form.nb_questions)
        .bind(updated_form.template)
        .bind(form_id)
        .fetch_one(db)
        .await
    {
        Ok(form) => Ok(form),
        Err(error) => Err(error),
    }
}

pub async fn delete_form_in_db(db: &PgPool, form_id: i32) -> Result<(), sqlx::error::Error> {
    match sqlx::query(QUERY_DELETE_FORM)
        .bind(form_id)
        .execute(db)
        .await
    {
        Ok(_) => Ok(()),
        Err(error) => Err(error),
    }
}