use crate::application::{
    dtos::requests::JoinRequest, input_ports::room_input_port::RoomInputPort,
};

pub struct RoomController<P: RoomInputPort> {
    input_port: P,
}

impl<P: RoomInputPort> RoomController<P> {
    pub fn new(input_port: P) -> Self {
        RoomController { input_port }
    }

    pub async fn join_room(&self, room_code: String, player: JoinRequest) {
        self.input_port.join_room(room_code, player).await;
    }

    pub async fn create_room(&self, room_code: String) {
        self.input_port.create_room(room_code).await;
    }

    pub async fn get_room(&self, room_code: String) {
        self.input_port.get_room(room_code).await;
    }
}
