use crate::{
    application::dtos::room_dto::RoomDTO,
    domain::{entities::room::Room, repositories::room_repository::RoomRepository},
};
use std::sync::Arc;
use worker::Env;

#[derive(Clone)]
pub struct KvRoomRepository {
    kv: Arc<Env>,
}

impl KvRoomRepository {
    pub fn new(kv: Arc<Env>) -> Self {
        Self { kv }
    }

    fn kv(&self) -> Option<worker::kv::KvStore> {
        self.kv.kv("rooms").ok()
    }
}

impl RoomRepository for KvRoomRepository {
    async fn get(&self, room_code: &str) -> Option<RoomDTO> {
        self.kv()?
            .get(room_code)
            .json::<Option<RoomDTO>>()
            .await
            .ok()
            .flatten()?
    }

    async fn save(&self, room: RoomDTO) -> Result<(), String> {
        self.kv()
            .ok_or_else(|| "Failed to access KV".to_string())?
            .put(&room.room_code, room.clone())
            .map_err(|_| "Failed to put room into KV".to_string())?
            .expiration_ttl(3600)
            .execute()
            .await
            .map_err(|_| "Failed to execute KV put".to_string())
    }

    async fn exists(&self, room_code: &str) -> Result<bool, String> {
        self.kv()
            .ok_or_else(|| "Failed to access KV".to_string())?
            .get(room_code)
            .json::<Option<Room>>()
            .await
            .map(|room| room.is_some())
            .map_err(|_| "Failed to retrieve room".to_string())
    }
}
