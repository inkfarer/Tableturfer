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

type ActionHandlerResult = Result<(), SocketError>;

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
        let result: ActionHandlerResult = {
            let mut room_store = self.state.room_store.write().await;
            if let Some(room) = room_store.get_mut(&self.room_code) {
                let auth_result = self.authorize_action(action.clone(), room);

                if auth_result.is_ok() {
                    match action {
                        SocketAction::SetMap(map) => room.set_map(map),
                        SocketAction::StartGame => room.start_game().await,
                        SocketAction::RequestRedraw => {
                            let team = self.team(room);
                            room.request_redraw(team.unwrap()).await
                        },
                        SocketAction::ProposeMove(player_move) => {
                            let team = self.team(room);
                            room.propose_move(team.unwrap(), player_move).await
                        },
                        SocketAction::SetDeck { id, cards } => room.set_deck(self.id, id, cards),
                        SocketAction::ReturnToRoom => {
                            room.return_to_room();
                            Ok(())
                        },
                        SocketAction::Ping => {
                            self.socket_channel.send(SocketEvent::Pong).await.ok();
                            Ok(())
                        },
                    }
                } else {
                    auth_result
                }
            } else {
                Err(SocketError::RoomNotFound(self.room_code.clone()))
            }
        };

        if let Err(err) = result {
            self.send_error(err).await.unwrap();
        }
    }

    fn authorize_action(&self, action: SocketAction, room: &Room) -> ActionHandlerResult {
        if action.is_owner_action() && room.owner_id != self.id {
            Err(SocketError::UserNotRoomOwner)
        } else if action.is_player_action()
            && room.owner_id != self.id
            && !room.is_opponent(self.id)
        {
            Err(SocketError::UserNotPlaying)
        } else {
            Ok(())
        }
    }

    fn team(&self, room: &Room) -> Option<PlayerTeam> {
        if room.owner_id == self.id {
            Some(PlayerTeam::Alpha)
        } else if room.opponent_id == Some(self.id) {
            Some(PlayerTeam::Bravo)
        } else {
            None
        }
    }

    pub async fn send_error(&self, err: SocketError) -> Result<(), SendError<SocketEvent>> {
        self.socket_channel.send(SocketEvent::Error(err)).await
    }
}
