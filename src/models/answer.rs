use serde::Deserialize;
use sqlx::FromRow;

#[derive(Deserialize, FromRow, Clone)]
pub struct Answer {
    pub id: i32,
    pub answer: String,
    pub template: String,
    pub question_id: i32,
    pub score: f64,
    pub engagement_score: f64,
    pub is_forced_engagement: bool,
    pub comment: String,
}

const QUERY_INSERT_ANSWER: &str = "
            INSERT INTO pfe.answers (answer, template, question_id, score, engagement_score, is_forced_engagement, comment) 
            VALUES ($1, $2, $3, $4, $5, $6, $7) 
            RETURNING id, answer, template, question_id, score, engagement_score, is_forced_engagement, comment
        ";

const QUERY_FIND_BY_QUESTION_ID_AND_ANSWER: &str = "
            SELECT id, answer, template, question_id, score, engagement_score, is_forced_engagement, comment
            FROM pfe.answers
            WHERE question_id = $1 & answer = $2
        ";

impl Answer {
    pub async fn create_answer(db: &sqlx::PgPool, answer: Answer) -> Result<Answer, sqlx::error::Error> {
        match sqlx::query_as::<_, Answer>(QUERY_INSERT_ANSWER)
            .bind(answer.answer.clone())
            .bind(answer.template.clone())
            .bind(answer.question_id.clone())
            .bind(answer.score.clone())
            .bind(answer.engagement_score.clone())
            .bind(answer.is_forced_engagement.clone())
            .bind(answer.comment.clone())
            .fetch_one(db)
            .await
        {
            Ok(answer) => Ok(answer),
            Err(error) => Err(error),
        }
    }

    pub async fn find_by_question_id_and_answer(db: &sqlx::PgPool, question_id: i32, answer: String) -> Result<Answer, sqlx::error::Error> {
        match sqlx::query_as::<_, Answer>(QUERY_FIND_BY_QUESTION_ID_AND_ANSWER)
            .bind(question_id)
            .bind(answer)
            .fetch_one(db)
            .await
        {
            Ok(answer) => {
                if answer.is_empty() {
                    Ok(Answer::default())
                } else {
                    Ok(answer[0].clone())
                }
            },
            Err(error) => Err(error),
        }
    }

    fn default() -> Answer {
        Answer {
            id: 0,
            answer: "".to_string(),
            template: "".to_string(),
            question_id: 0,
            score: 0.0,
            engagement_score: 0.0,
            is_forced_engagement: false,
            comment: "".to_string(),
        }
    }
    
}