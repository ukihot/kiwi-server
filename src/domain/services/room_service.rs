use crate::domain::{
    entities::room::Room, repositories::room_repository::RoomRepository,
    value_objects::room_code::RoomCode,
};

pub struct RoomService<R: RoomRepository> {
    pub repository: R,
}

impl<R: RoomRepository> RoomService<R> {
    pub fn new(repository: R) -> Self {
        RoomService { repository }
    }

    // 部屋コードがすでに存在するかを確認
    pub async fn exists(&self, room_code: &RoomCode) -> bool {
        self.repository
            .exists(room_code.value())
            .await
            .unwrap_or_default()
    }

    // 部屋を新規作成
    pub async fn create_room(&self, room_code: &RoomCode) -> Option<Room> {
        if self.exists(room_code).await {
            return None; // すでに存在する場合は `None` を返す
        }

        let room = Room::new(room_code.clone());

        match self.repository.save(room.clone().into()).await {
            Ok(_) => Some(room),
            Err(_) => None,
        }
    }
}
