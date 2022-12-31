use std::collections::HashMap;
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::game::state::{GameError, PlayerMove};
use crate::game::team::PlayerTeam;
use crate::socket::room_store::RoomUser;

#[derive(Serialize, Debug)]
#[serde(tag = "code", content = "detail")]
pub enum SocketError {
    MessageParsingFailed,
    UserNotRoomOwner,
    UserNotPlaying,
    RoomNotFound(String),
    MissingOpponent,
    RoomStarted,
    RoomNotStarted,
    DecksNotChosen,
    GameError(GameError),
}

#[derive(Deserialize, Clone)]
#[serde(tag = "action", content = "args")]
pub enum SocketAction {
    SetMap(String),
    StartGame,
    RequestRedraw,
    ProposeMove(PlayerMove),
    SetDeck { id: String, cards: IndexSet<String> },
    ReturnToRoom,
}

impl SocketAction {
    pub fn is_owner_action(&self) -> bool {
        match self {
            SocketAction::SetMap(_) => true,
            SocketAction::StartGame => true,
            SocketAction::ReturnToRoom => true,
            _ => false,
        }
    }

    pub fn is_player_action(&self) -> bool {
        match self {
            SocketAction::SetDeck { id: _, cards: _ } => true,
            SocketAction::ProposeMove(_) => true,
            SocketAction::RequestRedraw => true,
            _ => false,
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(tag = "event", content = "detail")]
pub enum SocketEvent {
    #[serde(rename_all = "camelCase")]
    Welcome {
        id: Uuid,
        room_code: String,
        users: HashMap<Uuid, RoomUser>,
        owner: Uuid,
        opponent: Option<Uuid>,
        map: String,
        started: bool,
    },
    Error(SocketError),
    RoomEvent(RoomEvent),
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "event", content = "detail")]
pub enum RoomEvent {
    UserJoin { id: Uuid, user: RoomUser },
    UserUpdate { id: Uuid, user: RoomUser },
    UserLeave(Uuid),
    OwnerChange(Uuid),
    OpponentChange(Option<Uuid>),
    MapChange(String),
    StartGame,
    #[serde(rename_all = "camelCase")]
    MoveReceived { team: PlayerTeam, remaining_turns: usize },
    MovesApplied(HashMap<PlayerTeam, PlayerMove>),
    HandAssigned(IndexSet<String>),
    #[serde(rename_all = "camelCase")]
    NextCardDrawn { new_card: String, replacing: String },
    EndGame { score: HashMap<PlayerTeam, usize> },
    ReturnToRoom,
}
