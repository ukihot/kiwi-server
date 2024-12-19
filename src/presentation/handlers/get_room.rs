use crate::{
    application::{dtos::responces::RoomResponse, interactors::room_interactor::RoomInteractor},
    infrastructure::room_kv::KvRoomRepository,
    presentation::{
        controllers::room_controller::RoomController, presenters::room_presenter::RoomPresenter,
    },
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use std::sync::Arc;
use worker::Env;

#[worker::send]
pub async fn get_room(
    Path(room_code): Path<String>,
    State(env): State<Arc<Env>>,
) -> impl IntoResponse {
    // Output Port（Presenter）を用意
    let presenter = RoomPresenter::<RoomResponse>::new();

    // Interactor を作成
    let repository = KvRoomRepository::new(env.clone());
    let interactor = RoomInteractor::new(repository.clone(), presenter.clone());

    // RoomController を作成
    let controller = RoomController::new(interactor);

    controller.get_room(room_code).await;
    presenter.response()
}
