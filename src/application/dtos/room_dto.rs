use serde::{Deserialize, Serialize};

use crate::domain::{
    entities::{player::Player, room::Room},
    value_objects::{player_name::PlayerName, room_code::RoomCode},
};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RoomDTO {
    pub room_code: String,
    pub players: Vec<String>,
}

impl From<Room> for RoomDTO {
    fn from(room: Room) -> Self {
        RoomDTO {
            room_code: room.room_code.to_string(),
            players: room.get_players(),
        }
    }
}

impl From<RoomDTO> for Result<Room, String> {
    fn from(room_dto: RoomDTO) -> Self {
        // room_code を RoomCode に変換。失敗した場合はエラーメッセージを返す
        let room_code = RoomCode::new(room_dto.room_code)
            .map_err(|e| format!("Failed to create RoomCode: {}", e))?;

        // players を Player の Vec に変換。失敗した場合はエラーメッセージを返す
        let players = room_dto
            .players
            .into_iter()
            .map(|player_name| {
                PlayerName::new(player_name)
                    .map(Player::new)
                    .map_err(|e| format!("Failed to create PlayerName: {}", e))
            })
            .collect::<Result<Vec<Player>, String>>()?;

        Ok(Room { room_code, players })
    }
}
