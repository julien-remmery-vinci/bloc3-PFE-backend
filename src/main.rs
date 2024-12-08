mod services;
mod routes;
mod database;
mod models;
mod errors;
mod middlewares;

use axum::body::Body;
use axum::extract::Request;
use axum::http::{
    header, 
    HeaderValue, 
    Method, 
    Response
};
use axum::{
    routing::get,
    routing::post,
    middleware,
    Router
};
use std::time::Duration;
use routes::forms::{
    create_form, 
    read_forms_by_company
};
use routes::answers::{
    create_answer, 
    create_answer_for_user, 
    read_answers_by_question
};
use routes::questions::{
    create_question, 
    read_one_question, 
    update_question
};
use routes::companies::get_company;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::Span;
use routes::auth::{
    login,
    register,
    verify
};

use database::state::AppState;
use crate::middlewares::authorization::{
    authorize_admin, 
    authorize_user
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let state = AppState::new().await;

    tracing_subscriber::fmt().init();

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
        .route("/forms", post(create_form)
            .layer(middleware::from_fn_with_state(state.clone(), authorize_admin)))
        .route("/forms/company", get(read_forms_by_company)
            .layer(middleware::from_fn_with_state(state.clone(), authorize_user)))
        // .route("/forms/:id", get(read_form)
        //     .put(update_form)
        //     .delete(delete_form))
        // .route("/forms/user/:id", get(read_forms_by_user))
        .route("/questions", post(create_question)
            .layer(middleware::from_fn_with_state(state.clone(), authorize_admin)))
        .route("/questions/:id", get(read_one_question)
            .layer(middleware::from_fn_with_state(state.clone(), authorize_user))
             .put(update_question)
            .layer(middleware::from_fn_with_state(state.clone(), authorize_admin)))
        .route("/answers", post(create_answer))
        .route("/answers/:id", post(create_answer_for_user)
            .layer(middleware::from_fn_with_state(state.clone(), authorize_user)))
        .route("/answers/:id", get(read_answers_by_question)
            .layer(middleware::from_fn_with_state(state.clone(), authorize_user)))
        .route("/company", get(get_company)
            .layer(middleware::from_fn_with_state(state.clone(), authorize_admin)))
        // .route("/answers/:id", get(read_one)
        //     .put(update)
        //     .delete(delete))
        .route("/score", get(read_score_for_form_user)
            .layer(middleware::from_fn_with_state(state.clone(), authorize_user)))
        .layer(cors)
        .layer(TraceLayer::new_for_http()
            .on_request(|request: &Request<Body>, _span: &Span| {
                tracing::info!(
                    method = %request.method(),
                    uri = %request.uri(),
                );
            })
            .on_response(|response: &Response<Body>, latency: Duration, _span: &Span| {
                tracing::info!(
                    status = %response.status(),
                    latency = ?latency,
                );
            }))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://0.0.0.0:3000");

    axum::serve(listener, app.into_make_service()).await.unwrap();
}