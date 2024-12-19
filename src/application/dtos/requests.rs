use serde::Deserialize;
#[derive(Deserialize)]
pub struct JoinRequest {
    pub player_name: String,
}
