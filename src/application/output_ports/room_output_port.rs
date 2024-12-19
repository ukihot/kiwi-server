use crate::application::dtos::room_dto::RoomDTO;

pub trait RoomOutputPort {
    fn on_room_joined(&self, room: RoomDTO);
    fn on_room_full(&self);
    fn on_room_not_found(&self);

    fn on_room_already_exists(&self);
    fn on_room_created(&self, room: RoomDTO);
    fn on_room_creation_failed(&self);

    fn on_room_save_failed(&self, err: String);

    fn on_room_found(&self, room: RoomDTO);

    fn on_invalid_room_code(&self, err: String); // 部屋コードが無効な場合
    fn on_invalid_player_name(&self); // プレイヤー名が無効な場合
}
