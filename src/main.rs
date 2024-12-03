use axum::{
    extract::{Json, State},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, FromRow, PgPool};
use std::sync::Arc;
use tokio::net::TcpListener;

#[derive(Debug, FromRow, Clone, Serialize)]
struct Hello {
    id: i32,
    text: String,
}

#[derive(Deserialize)]
struct ConnexionRequest {
    login: String,
    password: String,
}

#[derive(Serialize)]
struct ConnexionResponse {
    success: bool,
    message: String,
}

type SharedPool = Arc<PgPool>;

#[tokio::main]
async fn main() {
    // Connexion à la base de données
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://dev:password@postgres_db:5432/pfe_backend")
        .await
        .expect("Impossible de se connecter à la base");

    println!("Connexion à la base de données réussie.");

    let shared_pool = Arc::new(db_pool);

    // Build l'application avec des routes
    let app = Router::new()
        .route("/", get(get_hello))
        .route("/connexion", post(post_connexion))
        .with_state(shared_pool);

    // Création d'un TcpListener et démarrage du serveur
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://0.0.0.0:3000");

    // Lancer le serveur avec axum::serve
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

async fn get_hello(State(pool): State<SharedPool>) -> Json<Vec<Hello>> {
    let query = "SELECT id, text FROM pfe.hello";

    match sqlx::query_as::<_, Hello>(query).fetch_all(&*pool).await {
        Ok(hello) => Json(hello),
        Err(_) => {
            eprintln!("Error fetching hello");
            Json(vec![])
        }
    }
}

async fn post_connexion(
    Json(payload): Json<ConnexionRequest>,
    State(pool): State<SharedPool>,
) -> Json<ConnexionResponse> {
    // Requête pour vérifier l'utilisateur
    let query = "SELECT password FROM pfe.users WHERE email = $1";
    match sqlx::query(query)
        .bind(&payload.login)
        .fetch_one(&*pool)
        .await
    {
        Ok(row) => {
            let stored_password: String = row.try_get("password").unwrap();
            if stored_password == payload.password {
                Json(ConnexionResponse {
                    success: true,
                    message: "Connexion réussie".to_string(),
                })
            } else {
                Json(ConnexionResponse {
                    success: false,
                    message: "Mot de passe incorrect".to_string(),
                })
            }
        }
        Err(_) => Json(ConnexionResponse {
            success: false,
            message: "Utilisateur introuvable".to_string(),
        }),
    }
}
