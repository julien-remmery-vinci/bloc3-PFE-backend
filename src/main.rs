use axum::http::{header, HeaderValue, Method};
use axum::{
    routing::post,
    Router
};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

mod database;
use crate::database::database::AppState;
mod users;
mod auth;
use crate::auth::auth::login;

#[tokio::main]
async fn main() {
    let state = AppState::new().await;

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:4200".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

    let app = Router::new()
        .route("/auth/login", post(login))
        .layer(cors)
        .with_state(state);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on http://127.0.0.1:3000");

    axum::serve(listener, app.into_make_service()).await.unwrap();
}