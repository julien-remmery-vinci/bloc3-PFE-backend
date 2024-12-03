use axum::http::{header, HeaderValue, Method};
use axum::{
    routing::post,
    Router
};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

pub mod models;
pub mod routes;
use crate::routes::auth::login;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let state = AppState::new().await;

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:4200".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

    let app = Router::new()
        .route("/auth/login", post(login))
        .layer(cors)
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://0.0.0.0:3000");

    axum::serve(listener, app.into_make_service()).await.unwrap();
}