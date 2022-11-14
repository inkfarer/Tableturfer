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
    Welcome { room_code: String, users: HashMap<Uuid, RoomUser> },
    UserJoin { id: Uuid, user: RoomUser },
    UserLeave(Uuid),
    Broadcast { from: Uuid, message: String },
    Error(String),
}

impl From<RoomEvent> for SocketEvent {
    fn from(event: RoomEvent) -> Self {
        match event {
            RoomEvent::UserJoin { id, user } => {
                Self::UserJoin { id, user }
            }
            RoomEvent::UserLeave(id) => {
                Self::UserLeave(id)
            }
            RoomEvent::Broadcast { from, message } => {
                Self::Broadcast { from, message }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum RoomEvent {
    UserJoin { id: Uuid, user: RoomUser },
    UserLeave(Uuid),
    Broadcast { from: Uuid, message: String },
}
