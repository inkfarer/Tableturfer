use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::SendError;
use uuid::Uuid;
use crate::AppState;
use crate::game::team::PlayerTeam;
use crate::socket::messages::{SocketError, SocketEvent, SocketAction};
use crate::socket::room_store::Room;

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
                self.using_room_if_owner(|room| room.set_map(map)).await,
            SocketAction::StartGame =>
                self.using_room_if_owner(|room| room.start_game()).await,
            SocketAction::ProposeMove(player_move) =>
                self.using_room_if_player(|room, team| room.propose_move(team, player_move)).await,
            SocketAction::SetDeck(deck) =>
                self.using_room_if_player(|room, _| room.set_deck(self.id, deck)).await,
        }
    }

    async fn using_room_if_owner<F>(&self, action: F)
        where
            F: FnOnce(&mut Room) -> Result<(), SocketError>,
    {
        self.using_room(|room| {
            if room.owner_id == self.id {
                action(room)
            } else {
                Err(SocketError::UserNotRoomOwner)
            }
        }).await
    }

    async fn using_room_if_player<F>(&self, action: F)
        where
            F: FnOnce(&mut Room, PlayerTeam) -> Result<(), SocketError>,
    {
        self.using_room(|room| {
            if room.owner_id == self.id || room.is_opponent(self.id) {
                let team = if room.owner_id == self.id {
                    PlayerTeam::Alpha
                } else {
                    PlayerTeam::Bravo
                };

                action(room, team)
            } else {
                Err(SocketError::UserNotPlaying)
            }
        }).await
    }

    async fn using_room<F>(&self, action: F)
        where
            F: FnOnce(&mut Room) -> Result<(), SocketError>,
    {
        let action_result = {
            let mut room_store = self.state.room_store.write().unwrap();
            if let Some(room) = room_store.get_mut(&self.room_code) {
                action(room)
            } else {
                Err(SocketError::RoomNotFound(self.room_code.clone()))
            }
        };

        if let Err(err) = action_result {
            self.send_error(err).await.unwrap();
        }
    }

    pub async fn send_error(&self, err: SocketError) -> Result<(), SendError<SocketEvent>> {
        self.socket_channel.send(SocketEvent::Error(err)).await
    }
}
