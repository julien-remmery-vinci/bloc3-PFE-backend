use axum::http::{header, HeaderValue, Method};
use axum::{
    routing::get,
    routing::post,
    Router
};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
// use tower_http::trace::TraceLayer;
// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod models;
pub mod routes;
pub mod database;

use crate::routes::auth;
use crate::routes::forms;
use crate::routes::questions;
use crate::routes::answers;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let state = database::database::AppState::new().await;

    // tracing_subscriber::registry()
    //     .with(tracing_subscriber::fmt::layer())
    //     .init();

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:4200".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

    let app = Router::new()
        .route("/auth/login", post(auth::login))
        .route("/auth/register", post(auth::register))
        .route("/forms", post(forms::create))
        .route("/forms/:id", get(forms::read_one)
            .put(forms::update)
            .delete(forms::delete))
        .route("/forms/user/:id", get(forms::read_all))
        .route("/questions", post(questions::create))
        .route("/questions/:id", get(questions::read_one)
            .put(questions::update)
            .delete(questions::delete))
        .route("/answers", post(answers::create))
        .route("/answers/:id", get(answers::read_one)
            .put(answers::update)
            .delete(answers::delete))
        .layer(cors)
        // .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://0.0.0.0:3000");

    axum::serve(listener, app.into_make_service()).await.unwrap();
}