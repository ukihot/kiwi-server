use axum::{
    debug_handler,
    extract::{Path, State},
    http::Response,
    routing::{get, post},
    Json, Router,
};
use once_cell::sync::Lazy;
use rand::Rng;
use reqwest::header::{HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};
use tower_service::Service;
use uuid::Uuid;
use worker::*;

#[derive(Serialize, Deserialize, Clone)]
struct GameRoom {
    id: String,
    players: Vec<String>,
}

#[derive(Deserialize)]
struct JoinRequest {
    player_name: String,
}

type SharedState = Arc<Mutex<Vec<GameRoom>>>;

static SHARED_STATE: Lazy<SharedState> = Lazy::new(|| Arc::new(Mutex::new(Vec::new())));

#[event(fetch)]
async fn fetch(req: HttpRequest, _env: Env, _ctx: Context) -> Result<Response<axum::body::Body>> {
    // CORS設定
    let cors_options = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(["https://examples.com".parse::<HeaderValue>().unwrap()])
        .allow_headers([HeaderName::from_static("content-type")]);

    // APIルート設定
    let mut app = Router::new()
        .route("/create", post(create_room))
        .route("/state", get(get_room_state))
        .route("/join/:room_id", post(join_room))
        .layer(cors_options)
        .with_state(SHARED_STATE.clone());

    Ok(app.call(req).await?)
}

#[debug_handler]
async fn create_room(State(state): State<SharedState>) -> Json<GameRoom> {
    let mut rooms = state.lock().unwrap();
    let mut rng = rand::thread_rng();
    let buf: [u8; 16] = rng.gen();
    let new_room = GameRoom {
        id: Uuid::new_v8(buf).to_string(),
        players: vec![],
    };
    rooms.push(new_room.clone());
    Json(new_room)
}

#[debug_handler]
async fn get_room_state(State(state): State<SharedState>) -> Json<Vec<GameRoom>> {
    let rooms = state.lock().unwrap();
    Json(rooms.clone())
}

#[debug_handler]
async fn join_room(
    Path(room_id): Path<String>,
    State(state): State<SharedState>,
    Json(payload): Json<JoinRequest>,
) -> Json<Option<GameRoom>> {
    let mut rooms = state.lock().unwrap();
    if let Some(room) = rooms.iter_mut().find(|room| room.id == room_id) {
        room.players.push(payload.player_name);
        return Json(Some(room.clone()));
    }
    Json(None)
}
