use axum::{
    http::Response,
    routing::{get, post},
    Router,
};
use reqwest::header::{HeaderName, HeaderValue};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_service::Service;
use worker::{event, Context, Env, HttpRequest};

mod application;
mod domain;
mod infrastructure;
mod presentation;
use crate::domain::entities::player::Player;
use crate::presentation::handlers::create_room::create_room;
use crate::presentation::handlers::get_room::get_room;
use crate::presentation::handlers::join_room::join_room;

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> worker::Result<Response<axum::body::Body>> {
    // CORS
    let cors_options = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(["https://examples.com".parse::<HeaderValue>().unwrap()])
        .allow_headers([HeaderName::from_static("content-type")]);

    // エンドポイント
    let mut app = Router::new()
        .route("/create/:room_code", post(create_room))
        .route("/state/:room_code", get(get_room))
        .route("/join/:room_code", post(join_room))
        .layer(cors_options)
        .with_state(Arc::new(env));
    Ok(app.call(req).await?)
}
