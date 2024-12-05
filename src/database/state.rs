use sqlx::postgres::PgPoolOptions;
use std::env;

use crate::services::{auth::AuthService, questions::QuestionService, answers::AnswerService};

#[derive(Debug, Clone)]
pub struct AppState {
    pub auth: AuthService,
    pub question: QuestionService,
    pub answer: AnswerService,
}

impl AppState {
    pub async fn new() -> Self {
        let db = PgPoolOptions::new()
            .max_connections(5)
            .connect(
                env::var("DATABASE_URL")
                    .expect("DATABASE_URL must be set")
                    .as_str(),
            )
            .await
            .expect("Impossible de se connecter à la base");
        println!("connected to db");
        sqlx::migrate!("./migrations")
            .run(&db)
            .await.unwrap();
        Self {
            auth: AuthService { db:db.clone() },
            question: QuestionService { db: db.clone() },
            answer: AnswerService { db: db.clone() },
        }
    }
}
