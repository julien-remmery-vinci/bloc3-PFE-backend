pub mod database {
    use sqlx::postgres::PgPoolOptions;
    use std::env;
    #[derive(Debug, Clone)]
    pub struct AppState {
        pub db: sqlx::PgPool,
    }

    impl AppState {
        pub async fn new() -> Self {
            let db = PgPoolOptions::new()
                .max_connections(5)
                .connect(env::var("DATABASE_URL").expect("DB_URL must be set").as_str())
                .await
                .expect("Impossible de se connecter à la base");
            println!("connected to db");
            Self { db }
        }
    }
}