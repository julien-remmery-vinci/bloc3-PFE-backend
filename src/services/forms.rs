use crate::models::answers::Answer;
use crate::models::form::{Form, QuestionWithAnswers};
use crate::models::question::Question;

use sqlx::{Error, PgPool, Transaction};

const QUERY_INSERT_FORM: &str = "
    INSERT INTO pfe.forms (company, type, nb_questions, template)
    VALUES ($1, $2, $3, $4)
    RETURNING form_id, company, type AS form_type, nb_questions, template
";

const QUERY_INSERT_QUESTION: &str = "
    INSERT INTO pfe.questions (form_id, question_status, category, sub_category, question)
    VALUES ($1, $2, $3, $4, $5)
    RETURNING id, form_id, question_status, category, sub_category, question
";

const QUERY_INSERT_ANSWER: &str = "
    INSERT INTO pfe.answers (question_id, answer, template, score, engagement_score, is_forced_engagement, comment)
    VALUES ($1, $2, $3, $4, $5, $6, $7)
    RETURNING answer_id, question_id, answer, template, score, engagement_score, is_forced_engagement, comment
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

const QUERY_SELECT_ANSWERS_BY_QUESTION: &str = "
    SELECT 
        a.answer_id,
        a.answer,
        a.template,
        a.score,
        a.engagement_score,
        a.is_forced_engagement,
        a.comment
    FROM 
        pfe.answers a
    WHERE 
        a.question_id = $1
";

const QUERY_SELECT_QUESTIONS_BY_FORM: &str = "
    SELECT q.id AS question_id, q.category, q.sub_category, q.question, q.is_used, qf.question_status
    FROM pfe.questions_form qf
    JOIN pfe.questions q ON qf.question_id = q.id
    WHERE qf.form_id = $1
";

/// Créer un nouveau formulaire et insérer ses questions/réponses associées
pub async fn create_form_in_db(db: &PgPool, new_form: Form) -> Result<Form, Error> {
    let mut tx = db.begin().await?; // Démarrer une transaction

    // Insérer le formulaire
    let form = sqlx::query_as::<_, Form>(QUERY_INSERT_FORM)
        .bind(new_form.company)
        .bind(new_form.form_type)
        .bind(new_form.nb_questions)
        .bind(new_form.template)
        .fetch_one(&mut *tx)
        .await?;

    // Si des questions sont associées, les insérer
    if let Some(questions_json) = &new_form.questions {
        let questions_with_answers: Vec<QuestionWithAnswers> =
            serde_json::from_value(questions_json.clone())
                .map_err(|_| Error::Decode("Failed to deserialize questions".into()))?;

        for question_with_answers in questions_with_answers {
            let question = sqlx::query_as::<_, Question>(QUERY_INSERT_QUESTION)
                .bind(form.form_id.unwrap())
                .bind(&question_with_answers.question.category)
                .bind(&question_with_answers.question.sub_category)
                .bind(&question_with_answers.question.question)
                .bind(question_with_answers.question.is_used)
                .fetch_one(&mut *tx)
                .await?;

            // Insérer les réponses associées à la question
            for answer in question_with_answers.answers {
                sqlx::query_as::<_, Answer>(QUERY_INSERT_ANSWER)
                    .bind(question.id)
                    .bind(&answer.answer)
                    .bind(&answer.template)
                    .bind(answer.score)
                    .bind(answer.engagement_score)
                    .bind(answer.is_forced_engagement)
                    .bind(&answer.comment)
                    .fetch_one(&mut *tx)
                    .await?;
            }
        }
    }

    tx.commit().await?; // Commit de la transaction
    Ok(form)
}

/// Lire un formulaire par ID
pub async fn read_form_in_db(db: &PgPool, form_id: i32) -> Result<Form, Error> {
    let form = sqlx::query_as::<_, Form>(QUERY_SELECT_FORM)
        .bind(form_id)
        .fetch_one(db)
        .await?;

    let questions = sqlx::query_as::<_, Question>(QUERY_SELECT_QUESTIONS_BY_FORM)
        .bind(form_id)
        .fetch_all(db)
        .await?;

    let mut questions_with_answers = Vec::new();
    for question in questions {
        let answers = sqlx::query_as::<_, Answer>(QUERY_SELECT_ANSWERS_BY_QUESTION)
            .bind(question.id)
            .fetch_all(db)
            .await?;

        questions_with_answers.push(QuestionWithAnswers { question, answers });
    }

    Ok(Form {
        questions: Some(serde_json::to_value(questions_with_answers)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?),
        ..form
    })
    
}

/// Mettre à jour un formulaire
pub async fn update_form_in_db(
    db: &PgPool,
    form_id: i32,
    updated_form: Form,
) -> Result<Form, Error> {
    let form = sqlx::query_as::<_, Form>(QUERY_UPDATE_FORM)
        .bind(updated_form.company)
        .bind(updated_form.form_type)
        .bind(updated_form.nb_questions)
        .bind(updated_form.template)
        .bind(form_id)
        .fetch_one(db)
        .await?;

    Ok(form)
}

/// Supprimer un formulaire par ID
pub async fn delete_form_in_db(db: &PgPool, form_id: i32) -> Result<(), Error> {
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

/// Lire les formulaires d'un utilisateur
pub async fn read_forms_by_user_in_db(db: &PgPool, user_id: i32) -> Result<Vec<Form>, Error> {
    let forms = sqlx::query_as::<_, Form>(QUERY_SELECT_FORMS_BY_USER)
        .bind(user_id)
        .fetch_all(db)
        .await?;

    let mut forms_with_questions = Vec::new();
    for mut form in forms {
        let questions = sqlx::query_as::<_, Question>(QUERY_SELECT_QUESTIONS_BY_FORM)
            .bind(form.form_id)
            .fetch_all(db)
            .await?;

        let mut questions_with_answers = Vec::new();
        for question in questions {
            let answers = sqlx::query_as::<_, Answer>(QUERY_SELECT_ANSWERS_BY_QUESTION)
                .bind(question.id)
                .fetch_all(db)
                .await?;

            questions_with_answers.push(QuestionWithAnswers { question, answers });
        }

        form.questions = Some(serde_json::to_value(questions_with_answers)
    .map_err(|e| sqlx::Error::Decode(Box::new(e)))?);
        forms_with_questions.push(form);
    }

    Ok(forms_with_questions)
}
