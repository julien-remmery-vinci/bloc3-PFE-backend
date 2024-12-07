use sqlx::postgres::PgPoolOptions;
use std::env;

use crate::services::{answers::AnswerService, auth::AuthService, company::CompanyService, forms::FormService, questions::QuestionService};

#[derive(Debug, Clone)]
pub struct AppState {
    pub auth: AuthService,
    pub form: FormService,
    pub question: QuestionService,
    pub company: CompanyService,
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
            .expect("Impossible de se connecter à la base de données");
        println!("Connexion à la base de données établie");

        sqlx::migrate!("./migrations")
            .run(&db)
            .await
            .expect("Erreur lors de la migration de la base de données");
        println!("Migration effectuée avec succès");
        
        Self {
            auth: AuthService { db: db.clone() },
            form: FormService { db: db.clone() },
            question: QuestionService { db: db.clone() },
            company: CompanyService { db: db.clone() },
            answer: AnswerService { db: db.clone() },
        }
    }
}
