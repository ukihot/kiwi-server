use crate::application::dtos::responces::RoomResponse;
use crate::application::interactors::room_interactor::RoomInteractor;
use crate::presentation::controllers::room_controller::RoomController;
use crate::presentation::presenters::room_presenter::RoomPresenter;
use crate::{application::dtos::requests::JoinRequest, infrastructure::room_kv::KvRoomRepository};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use worker::Env;

#[worker::send]
pub async fn join_room(
    Path(room_code): Path<String>,
    State(env): State<Arc<Env>>,
    Json(player): Json<JoinRequest>,
) -> impl IntoResponse {
    // Output Port（Presenter）を用意
    let presenter = RoomPresenter::<RoomResponse>::new();

    // Interactor を作成
    let repository = KvRoomRepository::new(env.clone());
    let interactor = RoomInteractor::new(repository, presenter.clone());

    // RoomController を作成
    let controller = RoomController::new(interactor);

    controller.join_room(room_code, player).await;
    presenter.response()
}
