use crate::models::form::Form;
use sqlx::PgPool;

const QUERY_INSERT_FORM: &str = "
    INSERT INTO pfe.forms (company, type, nb_questions, template)
    VALUES ($1, $2, $3, $4)
    RETURNING form_id, company, type AS form_type, nb_questions, template
";

const QUERY_SELECT_FORM: &str = "
    SELECT form_id, company, type AS form_type, nb_questions, template
    FROM pfe.forms
    WHERE form_id = $1
";

const QUERY_UPDATE_FORM: &str = "
    UPDATE pfe.forms
    SET company = $1, type = $2, nb_questions = $3, template = $4
    WHERE form_id = $5
    RETURNING form_id, company, type AS form_type, nb_questions, template
";

const QUERY_DELETE_FORM: &str = "
    DELETE FROM pfe.forms
    WHERE form_id = $1
    ";

const QUERY_SELECT_FORMS_BY_USER: &str = "
    SELECT form_id, company, type AS form_type, nb_questions, template
    FROM pfe.forms
    WHERE company = $1
";

pub async fn create_form_in_db(db: &PgPool, new_form: Form) -> Result<Form, sqlx::error::Error> {
    match sqlx::query_as::<_, Form>(QUERY_INSERT_FORM)
        .bind(new_form.company)
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

pub async fn update_form_in_db(
    db: &PgPool,
    form_id: i32,
    updated_form: Form,
) -> Result<Form, sqlx::error::Error> {
    match sqlx::query_as::<_, Form>(QUERY_UPDATE_FORM)
        .bind(updated_form.company)
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
    let result = sqlx::query(QUERY_DELETE_FORM)
        .bind(form_id)
        .execute(db)
        .await?;

    if result.rows_affected() == 0 {
        Err(sqlx::Error::RowNotFound)
    } else {
        Ok(())
    }
}

pub async fn read_forms_by_user_in_db(db: &PgPool, user_id: i32) -> Result<Vec<Form>, sqlx::error::Error> {
    let forms = sqlx::query_as::<_, Form>(QUERY_SELECT_FORMS_BY_USER)
        .bind(user_id)
        .fetch_all(db)
        .await?;

    if forms.is_empty() {
        Err(sqlx::Error::RowNotFound)
    } else {
        Ok(forms)
    }
}
