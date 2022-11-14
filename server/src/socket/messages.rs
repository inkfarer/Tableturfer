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
    UserJoin { id: Uuid, user: RoomUser },
    UserLeave(Uuid),
    OwnerChange(Uuid),
    Broadcast { from: Uuid, message: String },
    Error(String),
}

impl From<RoomEvent> for SocketEvent {
    fn from(event: RoomEvent) -> Self {
        match event {
            RoomEvent::UserJoin { id, user } => Self::UserJoin { id, user },
            RoomEvent::Broadcast { from, message } => Self::Broadcast { from, message },
            RoomEvent::UserLeave(id) => Self::UserLeave(id),
            RoomEvent::OwnerChange(id) => Self::OwnerChange(id)
        }
    }
}

#[derive(Clone, Debug)]
pub enum RoomEvent {
    UserJoin { id: Uuid, user: RoomUser },
    UserLeave(Uuid),
    Broadcast { from: Uuid, message: String },
    OwnerChange(Uuid),
}
