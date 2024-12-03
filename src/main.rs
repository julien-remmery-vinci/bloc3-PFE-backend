use axum::{
    extract::State,
    routing::get,
    Router,
    Json,
};
use sqlx::{postgres::PgPoolOptions, FromRow, PgPool};
use std::sync::Arc;
use tokio::net::TcpListener;

#[derive(Debug, FromRow, Clone, serde::Serialize)]
struct Hello {
    id: i32,
    text: String,
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

    // Tester un SELECT simple pour vérifier que la table "pfe.hello" existe
    let test_query = "SELECT id, text FROM pfe.hello LIMIT 1";
    match sqlx::query(test_query).fetch_optional(&db_pool).await {
        Ok(Some(_)) => println!("Test de base de données réussi : la table 'pfe.hello' est accessible."),
        Ok(None) => println!("La table 'pfe.hello' est vide, mais accessible."),
        Err(e) => {
            eprintln!("Erreur lors du test de la base de données : {:?}", e);
            return; // Arrêter l'application si la base de données ne fonctionne pas
        }
    }

    let shared_pool = Arc::new(db_pool);

    // Build l'application avec une route
    let app = Router::new()
        .route("/", get(get_hello))
        .with_state(shared_pool);

    // Création d'un TcpListener et démarrage du serveur
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://0.0.0.0:3000");

    // Lancer le serveur avec axum::serve
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

// Fonction handler pour la route "/"
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
