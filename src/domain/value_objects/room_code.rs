use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RoomCode(String);

impl RoomCode {
    pub fn new(code: String) -> Result<Self, String> {
        if code.len() > 10 {
            return Err("Room code exceeds the maximum length of 10 characters.".to_string());
        }

        if !code.chars().all(|c| c.is_alphanumeric()) {
            return Err("Room code must contain only alphanumeric characters.".to_string());
        }

        Ok(Self(code))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RoomCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
