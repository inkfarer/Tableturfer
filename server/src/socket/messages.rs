use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::socket::room_store::RoomUser;

#[derive(Deserialize)]
#[serde(tag = "action", content = "args")]
pub enum SocketRequest {
    Broadcast(String),
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
    },
    Error(String),
    RoomEvent(RoomEvent),
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "event", content = "detail")]
pub enum RoomEvent {
    UserJoin { id: Uuid, user: RoomUser },
    UserLeave(Uuid),
    Broadcast { from: Uuid, message: String },
    OwnerChange(Uuid),
}
