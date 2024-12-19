use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerName(String);

impl PlayerName {
    pub fn new(name: String) -> Result<Self, String> {
        if name.len() > 30 {
            return Err("Player name exceeds the maximum length of 30 characters.".to_string());
        }

        if !name.chars().all(|c| c.is_alphanumeric()) {
            return Err("Player name must contain only alphanumeric characters.".to_string());
        }

        Ok(Self(name))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PlayerName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
