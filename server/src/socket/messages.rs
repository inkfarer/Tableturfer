use std::collections::HashMap;
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::game::map::GameMap;
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
    SetMap(GameMap),
    StartGame,
    ProposeMove(PlayerMove),
    SetDeck(IndexSet<String>),
}

impl SocketAction {
    pub fn is_owner_action(&self) -> bool {
        match self {
            SocketAction::SetMap(_) => true,
            SocketAction::StartGame => true,
            _ => false,
        }
    }

    pub fn is_player_action(&self) -> bool {
        match self {
            SocketAction::SetDeck(_) => true,
            SocketAction::ProposeMove(_) => true,
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
        map: GameMap,
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
    MapChange(GameMap),
    StartGame,
    MoveReceived(PlayerTeam),
    MovesApplied(HashMap<PlayerTeam, PlayerMove>),
    HandAssigned(IndexSet<String>),
}
