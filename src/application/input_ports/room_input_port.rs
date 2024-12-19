use crate::application::dtos::requests::JoinRequest;

pub trait RoomInputPort {
    async fn join_room(&self, room_code: String, player: JoinRequest);
    async fn create_room(&self, room_code: String);
    async fn get_room(&self, room_code: String);
}
