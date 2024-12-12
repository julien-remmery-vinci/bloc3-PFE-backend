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
use axum::routing::patch;
use axum::{
    routing::get,
    routing::post,
    middleware::from_fn_with_state,
    Router
};
use routes::onboarding::{accept_onboarding, read_all_onboarding, read_all_pending_onboarding, read_all_rejected_onboarding, reject_onboarding, submit_onboarding};
use routes::stats::get_stats;
use std::time::Duration;
use routes::forms::{
    create_form, 
    read_forms_by_user, read_forms_with_questions_and_answers, submit_form, submit_validated_form
};
use routes::answers::{
    create_answer, 
    create_answer_for_user, 
    read_answers_by_question, 
    validate_user_answer, 
    update_answer_score
};
use routes::questions::{
    create_question, 
    read_one_question, 
    update_question
};
use routes::companies::{
    company_forms_status, create_company, get_user_company, read_all_companies, read_one_company
};
use routes::scores::sum_score_template;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::Span;
use routes::auth::{
    login, register, verify
};

use database::state::AppState;
use crate::middlewares::authorization::{
    authorize_admin, 
    authorize_user
};

fn auth_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/auth/login", post(login))
        .route("/auth/register", post(register)
        .layer(from_fn_with_state(state.clone(), authorize_user)))
        .route("/auth/verify", get(verify)
        .layer(from_fn_with_state(state.clone(), authorize_user)))
}

fn forms_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/forms", get(read_forms_with_questions_and_answers)
        .post(create_form)
        .layer(from_fn_with_state(state.clone(), authorize_admin)))
        .route("/forms/user",get(read_forms_by_user)
        .layer(from_fn_with_state(state.clone(), authorize_user)))
        .route("/forms/:id/submit", post(submit_form)
        .layer(from_fn_with_state(state.clone(), authorize_user)))
        .route("/forms/:id/validate", post(submit_validated_form)
        .layer(from_fn_with_state(state.clone(), authorize_admin)))
}

fn questions_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/questions",post(create_question)
        .layer(from_fn_with_state(state.clone(), authorize_admin)))
        .route("/questions/:id",get(read_one_question)
        .layer(from_fn_with_state(state.clone(), authorize_user))
        .put(update_question)
        .layer(from_fn_with_state(state.clone(), authorize_admin)))
}

fn answers_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/answers", post(create_answer)
        .layer(from_fn_with_state(state.clone(), authorize_admin)))
        .route("/answers/:id",post(create_answer_for_user)
        .layer(from_fn_with_state(state.clone(), authorize_user)),)
        .route("/answers/:id",get(read_answers_by_question)
        .layer(from_fn_with_state(state.clone(), authorize_user)))
        .route("/answers/:id/validate", post(validate_user_answer)
        .layer(from_fn_with_state(state.clone(), authorize_admin)))
        .route("/answers/:id/update-score", patch(update_answer_score)
        .layer(from_fn_with_state(state.clone(), authorize_admin)))
}

fn company_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/company",get(read_all_companies)
        .layer(from_fn_with_state(state.clone(), authorize_admin))
        .post(create_company)
        .layer(from_fn_with_state(state.clone(), authorize_admin)))
        .route("/company/:id", get(read_one_company)
        .layer(from_fn_with_state(state.clone(), authorize_admin)))
        .route("/company/user", get(get_user_company)
        .layer(from_fn_with_state(state.clone(), authorize_user)))
        .route("/company/:id/forms/status", get(company_forms_status)
        .layer(from_fn_with_state(state.clone(), authorize_user)))
}

fn score_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/score/:form_id", get(sum_score_template)
        .layer(from_fn_with_state(state.clone(), authorize_user)))
}

fn onboardings_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/onboarding", post(submit_onboarding))
        .route("/onboarding", get(read_all_onboarding)
        .layer(from_fn_with_state(state.clone(), authorize_admin)))
        .route("/onboarding/pending", get(read_all_pending_onboarding)
        .layer(from_fn_with_state(state.clone(), authorize_admin)))
        .route("/onboarding/rejected", get(read_all_rejected_onboarding)
        .layer(from_fn_with_state(state.clone(), authorize_admin)))
        .route("/onboarding/:id/accept", post(accept_onboarding)
        .layer(from_fn_with_state(state.clone(), authorize_admin)))
        .route("/onboarding/:id/reject", post(reject_onboarding)
        .layer(from_fn_with_state(state.clone(), authorize_admin)))
}

fn stats_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/stats", get(get_stats)
        .layer(from_fn_with_state(state.clone(), authorize_admin)))
}

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
            Method::OPTIONS,
            Method::PATCH
        ])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

        let app = Router::new()
            .merge(auth_routes(state.clone()))
            .merge(forms_routes(state.clone()))
            .merge(questions_routes(state.clone()))
            .merge(answers_routes(state.clone()))
            .merge(company_routes(state.clone()))
            .merge(score_routes(state.clone()))
            .merge(onboardings_routes(state.clone()))
            .merge(stats_routes(state.clone()))
            .layer(cors)
            .layer(
                TraceLayer::new_for_http()
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
                    }),
            )
            .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://0.0.0.0:3000");

    axum::serve(listener, app.into_make_service()).await.unwrap();
}