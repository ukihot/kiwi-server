use crate::{domain::value_objects::room_code::RoomCode, Player};
use serde::{Deserialize, Serialize};

// 部屋情報
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Room {
    pub room_code: RoomCode,  // ユーザー向けの識別子
    pub players: Vec<Player>, // 部屋に所属するプレイヤー
}

impl Room {
    // 部屋が満室かどうかをチェック
    pub fn is_full(&self) -> bool {
        self.players.len() >= 3 // 部屋に最大3人のプレイヤー
    }

    // プレイヤーを部屋に追加
    pub fn add_player(&mut self, player: Player) {
        if !self.is_full() {
            self.players.push(player);
        }
    }

    // 部屋の状態を取得
    pub fn get_players(&self) -> Vec<String> {
        self.players
            .iter()
            .map(|p| p.player_name.value().to_string())
            .collect()
    }
}

impl Room {
    pub fn new(room_code: RoomCode) -> Self {
        Self {
            room_code,
            players: Vec::new(),
        }
    }
}
