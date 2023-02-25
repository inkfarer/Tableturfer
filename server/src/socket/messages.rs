use std::collections::HashMap;
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::game::state::{GameError, PlayerMove};
use crate::game::team::PlayerTeam;
use crate::socket::room_store::{RoomConfig, RoomUser};

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
    Ping,
    SetConfig(RoomConfig),
}

impl SocketAction {
    pub fn is_owner_action(&self) -> bool {
        matches!(self, SocketAction::SetMap(_) | SocketAction::StartGame | SocketAction::ReturnToRoom | SocketAction::SetConfig(_))
    }

    pub fn is_player_action(&self) -> bool {
        matches!(self, SocketAction::SetDeck { id: _, cards: _ } | SocketAction::ProposeMove(_) | SocketAction::RequestRedraw)
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
        config: RoomConfig,
    },
    Error(SocketError),
    RoomEvent(RoomEvent),
    Pong,
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
    #[serde(rename_all = "camelCase")]
    StartGame { score: HashMap<PlayerTeam, usize>, map_name: String },
    #[serde(rename_all = "camelCase")]
    MoveReceived { team: PlayerTeam, remaining_turns: usize },
    MovesApplied { moves: HashMap<PlayerTeam, PlayerMove>, score: HashMap<PlayerTeam, usize> },
    HandAssigned(IndexSet<String>),
    #[serde(rename_all = "camelCase")]
    NextCardDrawn { new_card: String, replacing: String },
    EndGame { score: HashMap<PlayerTeam, usize> },
    ReturnToRoom,
    ConfigUpdate(RoomConfig),
}
