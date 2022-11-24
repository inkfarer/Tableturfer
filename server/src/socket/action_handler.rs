use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::SendError;
use uuid::Uuid;
use crate::AppState;
use crate::socket::messages::{SocketError, SocketEvent, SocketAction};
use crate::socket::room_store::SocketRoomStore;

pub struct SocketActionHandler {
    id: Uuid,
    state: Arc<AppState>,
    socket_channel: mpsc::Sender<SocketEvent>,
    room_code: String,
}

impl SocketActionHandler {
    pub fn new(id: Uuid, socket_channel: mpsc::Sender<SocketEvent>, state: Arc<AppState>, room_code: String) -> Self {
        Self {
            id,
            state,
            socket_channel,
            room_code
        }
    }

    pub async fn handle_action(&self, action: SocketAction) {
        match action {
            SocketAction::SetMap(map) =>
                self.using_room_store(|s| s.set_map(self.id, &self.room_code, map)).await,
            SocketAction::StartGame =>
                self.using_room_store(|s| s.start_room(self.id, &self.room_code)).await,
            SocketAction::ProposeMove(player_move) =>
                self.using_room_store(|s| s.propose_move(self.id, &self.room_code, player_move)).await
        }
    }

    async fn using_room_store<F>(&self, action: F)
        where
            F: FnOnce(&mut SocketRoomStore) -> Result<(), SocketError>,
    {
        let action_result = {
            let mut room_store = self.state.room_store.write().unwrap();
            action(&mut room_store)
        };

        if let Err(err) = action_result {
            self.send_error(err).await.unwrap();
        }
    }

    pub async fn send_error(&self, err: SocketError) -> Result<(), SendError<SocketEvent>> {
        self.socket_channel.send(SocketEvent::Error(err)).await
    }
}
