use axum::{
    routing::get,
    routing::post,
    Router
};
use tokio::net::TcpListener;

mod database;
use crate::database::database::AppState;
mod users;
mod auth;
use crate::auth::auth::login;

#[tokio::main]
async fn main() {
    // Connexion à la base de données
    let state = AppState::new().await;

    // Build l'application avec une route
    let app = Router::new()
        .route("/auth/login", post(login))
        .with_state(state);

    // Création d'un TcpListener et démarrage du serveur
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://0.0.0.0:3000");

    // Lancer le serveur avec axum::serve
    axum::serve(listener, app.into_make_service()).await.unwrap();
}