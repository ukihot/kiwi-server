use serde::Serialize;

use super::room_dto::RoomDTO;

pub trait KiwiResponse: Serialize {}

#[derive(Serialize, Clone)]
pub struct RoomResponse {
    pub room: RoomDTO,
    pub message: String,
}

impl KiwiResponse for RoomResponse {}
