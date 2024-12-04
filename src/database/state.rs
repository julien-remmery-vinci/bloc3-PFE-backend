use sqlx::postgres::PgPoolOptions;
use std::env;

use crate::services::auth::AuthService;

#[derive(Debug, Clone)]
pub struct AppState {
    pub auth: AuthService,
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
        Self {
            auth: AuthService { db },
        }
    }
}
