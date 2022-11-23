use std::collections::HashMap;
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
    GameError(GameError),
}

#[derive(Deserialize)]
#[serde(tag = "action", content = "args")]
pub enum SocketRequest {
    Broadcast(String),
    SetMap(GameMap),
    StartGame,
    ProposeMove(PlayerMove),
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
    UserLeave(Uuid),
    Broadcast { from: Uuid, message: String },
    OwnerChange(Uuid),
    OpponentChange(Option<Uuid>),
    MapChange(GameMap),
    StartGame,
    MoveReceived(PlayerTeam),
    MovesApplied(HashMap<PlayerTeam, PlayerMove>)
}
