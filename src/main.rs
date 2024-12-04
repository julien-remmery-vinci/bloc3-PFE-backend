use axum::http::{header, HeaderValue, Method};
use axum::{
    routing::get,
    routing::post,
    middleware,
    Router
};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
// use tower_http::trace::TraceLayer;
// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use routes::auth::login;
use routes::auth::register;
use services::auth::authorization_middleware;
use database::state::AppState;

mod services;
mod routes;
mod database;
mod models;
mod errors;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let state = AppState::new().await;

    // tracing_subscriber::registry()
    //     .with(tracing_subscriber::fmt::layer())
    //     .init();

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:4200".parse::<HeaderValue>().unwrap())
        .allow_methods([
            Method::GET, 
            Method::POST, 
            Method::PUT, 
            Method::DELETE, 
            Method::OPTIONS
        ])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" })
            .layer(middleware::from_fn_with_state(state.clone(), authorization_middleware))) // Exemple de middleware
        .route("/auth/login", post(login))
        .route("/auth/register", post(register))
        // .route("/forms", post(create))
        // .route("/forms/:id", get(read_one)
        //     .put(update)
        //     .delete(delete))
        // .route("/forms/user/:id", get(read_all))
        // .route("/questions", post(create))
        // .route("/questions/:id", get(read_one)
        //     .put(update)
        //     .delete(delete))
        // .route("/answers", post(create))
        // .route("/answers/:id", get(read_one)
        //     .put(update)
        //     .delete(delete))
        .layer(cors)
        // .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://0.0.0.0:3000");

    axum::serve(listener, app.into_make_service()).await.unwrap();
}