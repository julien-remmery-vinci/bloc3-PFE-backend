pub mod users {
    use sqlx::FromRow;
    #[derive(Debug, FromRow, Clone, serde::Serialize)]
    pub struct User {
        pub login: String,
        pub password: String
    }
}