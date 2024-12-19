use crate::{
    application::{
        dtos::requests::JoinRequest, input_ports::room_input_port::RoomInputPort,
        output_ports::room_output_port::RoomOutputPort,
    },
    domain::{
        entities::player::Player,
        repositories::room_repository::RoomRepository,
        services::room_service::RoomService,
        value_objects::{player_name::PlayerName, room_code::RoomCode},
    },
};

pub struct RoomInteractor<R: RoomRepository, O: RoomOutputPort> {
    repository: R,
    output_port: O,
    service: RoomService<R>,
}

impl<R: RoomRepository + Clone, O: RoomOutputPort> RoomInteractor<R, O> {
    pub fn new(repository: R, output_port: O) -> Self {
        let service = RoomService::new(repository.clone());
        Self {
            repository,
            output_port,
            service,
        }
    }
}

impl<R: RoomRepository, O: RoomOutputPort> RoomInputPort for RoomInteractor<R, O> {
    async fn join_room(&self, room_code: String, player: JoinRequest) {
        // PlayerNameの生成とエラーハンドリング
        let player_name = match PlayerName::new(player.player_name) {
            Ok(name) => name,
            Err(_) => {
                self.output_port.on_invalid_player_name();
                return;
            }
        };

        // 部屋の取得と処理
        match self.repository.get(&room_code).await {
            Some(room_dto) => {
                // RoomDTOからRoomへ変換 (エラーハンドリング)
                let mut room = match room_dto.into() {
                    Ok(r) => r,
                    Err(err) => {
                        self.output_port.on_invalid_room_code(err);
                        return;
                    }
                };

                // 部屋が満員でない場合
                if !room.is_full() {
                    room.add_player(Player::new(player_name));

                    // 部屋情報を保存
                    if let Err(err) = self.repository.save(room.clone().into()).await {
                        self.output_port.on_room_save_failed(err);
                        return;
                    }
                    self.output_port.on_room_joined(room.into());
                } else {
                    self.output_port.on_room_full();
                }
            }
            None => self.output_port.on_room_not_found(),
        }
    }

    async fn create_room(&self, room_code: String) {
        // RoomCodeを値オブジェクトとして生成
        let room_code = match RoomCode::new(room_code) {
            Ok(code) => code,
            Err(err) => {
                self.output_port.on_invalid_room_code(err);
                return;
            }
        };

        // 部屋の存在チェック
        if self.service.exists(&room_code).await {
            self.output_port.on_room_already_exists();
            return;
        }

        // 部屋の作成と保存
        match self.service.create_room(&room_code.clone()).await {
            Some(room) => {
                if let Err(err) = self.repository.save(room.clone().into()).await {
                    self.output_port.on_room_save_failed(err);
                    return;
                }
                self.output_port.on_room_created(room.into());
            }
            None => self.output_port.on_room_creation_failed(),
        }
    }

    async fn get_room(&self, room_code: String) {
        match self.repository.get(&room_code).await {
            Some(room) => self.output_port.on_room_found(room),
            None => self.output_port.on_room_not_found(),
        }
    }
}
