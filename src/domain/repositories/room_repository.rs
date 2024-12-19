use crate::application::dtos::room_dto::RoomDTO;

pub trait RoomRepository {
    async fn get(&self, room_code: &str) -> Option<RoomDTO>;
    async fn save(&self, room: RoomDTO) -> Result<(), String>;
    async fn exists(&self, room_code: &str) -> Result<bool, String>;
}
