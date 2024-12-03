pub mod database {
    use sqlx::postgres::PgPoolOptions;
    #[derive(Debug, Clone)]
    pub struct AppState {
        pub db: sqlx::PgPool,
    }

    impl AppState {
        pub async fn new() -> Self {
            let db = PgPoolOptions::new()
                .max_connections(5)
                .connect("postgres://dev:password@localhost:5432/postgres")
                .await
                .expect("Impossible de se connecter à la base");
            println!("connected to db");
            Self { db }
        }
    }
}