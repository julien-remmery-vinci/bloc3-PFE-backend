mod services;
mod routes;
mod database;
mod models;
mod errors;
mod authorization;

use axum::http::{header, HeaderValue, Method};
use axum::{
    routing::get,
    routing::post,
    routing::put,
    middleware,
    Router
};
use routes::answers::{create_answer, create_answer_for_user};
use routes::questions::{create_question, read_one_question, update_question};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
// use tower_http::trace::TraceLayer;
// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use routes::auth::{
    login,
    register,
    verify
};
use routes::forms::create_form;
use database::state::AppState;
use authorization::{
    authorize_user,
    authorize_admin
};



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
        .route("/auth/login", post(login))
        .route("/auth/register", post(register)
            .layer(middleware::from_fn_with_state(state.clone(), authorize_admin)))
        .route("/auth/verify", post(verify)
            .layer(middleware::from_fn_with_state(state.clone(), authorize_user)))
        .route("/forms", post(create_form))
        // .route("/forms/:id", get(read_one)
        //     .put(update)
        //     .delete(delete))
        // .route("/forms/user/:id", get(read_all))
        .route("/questions", post(create_question)
            .layer(middleware::from_fn_with_state(state.clone(), authorize_admin)))
        .route("/questions/:id", get(read_one_question)
            .layer(middleware::from_fn_with_state(state.clone(), authorize_user))
             .put(update_question)
             .layer(middleware::from_fn_with_state(state.clone(), authorize_admin)))
        //     .delete(delete))
        .route("/answers", post(create_answer))
        .route("/answers/:id", post(create_answer_for_user))
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