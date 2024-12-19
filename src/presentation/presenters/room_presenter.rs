use crate::application::{
    dtos::{
        responces::{KiwiResponse, RoomResponse},
        room_dto::RoomDTO,
    },
    output_ports::room_output_port::RoomOutputPort,
};
use axum::http::StatusCode;
use axum::{response::IntoResponse, Json};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct RoomPresenter<T: KiwiResponse> {
    response: Arc<Mutex<Option<Result<T, String>>>>,
}

impl<T: KiwiResponse + Clone> RoomPresenter<T> {
    pub fn new() -> Self {
        RoomPresenter {
            response: Arc::new(Mutex::new(None)),
        }
    }

    pub fn response(&self) -> impl IntoResponse {
        let response = self.response.lock().unwrap().clone();
        match response {
            Some(Ok(data)) => Json(data).into_response(),
            Some(Err(error)) => (StatusCode::BAD_REQUEST, Json(error)).into_response(),
            None => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unknown error".to_string(),
            )
                .into_response(),
        }
    }
}

impl RoomOutputPort for RoomPresenter<RoomResponse> {
    fn on_room_joined(&self, room: RoomDTO) {
        let response = RoomResponse {
            room,
            message: "Player successfully joined".to_string(),
        };
        *self.response.lock().unwrap() = Some(Ok(response));
    }

    fn on_room_full(&self) {
        *self.response.lock().unwrap() = Some(Err("Room is full".to_string()));
    }

    fn on_room_not_found(&self) {
        *self.response.lock().unwrap() = Some(Err("Room not found".to_string()));
    }

    fn on_room_already_exists(&self) {
        *self.response.lock().unwrap() = Some(Err("Room already exists".to_string()));
    }

    fn on_room_created(&self, room: RoomDTO) {
        let response = RoomResponse {
            room,
            message: "Room successfully created".to_string(),
        };
        *self.response.lock().unwrap() = Some(Ok(response));
    }

    fn on_room_creation_failed(&self) {
        *self.response.lock().unwrap() = Some(Err("Failed to create room".to_string()));
    }

    fn on_room_save_failed(&self, err: String) {
        *self.response.lock().unwrap() = Some(Err(err));
    }

    fn on_room_found(&self, room: RoomDTO) {
        let response = RoomResponse {
            room,
            message: "Room found successfully".to_string(),
        };
        *self.response.lock().unwrap() = Some(Ok(response));
    }

    fn on_invalid_room_code(&self, err: String) {
        *self.response.lock().unwrap() = Some(Err(err));
    }

    fn on_invalid_player_name(&self) {
        *self.response.lock().unwrap() = Some(Err("Invalid player name".to_string()));
    }
}
