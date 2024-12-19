use serde::{Deserialize, Serialize};

use crate::domain::value_objects::player_name::PlayerName;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Player {
    pub player_name: PlayerName,
}

impl Player {
    pub fn new(player_name: PlayerName) -> Self {
        Self { player_name }
    }
}
