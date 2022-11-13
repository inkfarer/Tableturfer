use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(tag = "action", content = "args")]
pub enum SocketRequest {
    Broadcast(String),
}

#[derive(Serialize, Debug)]
#[serde(tag = "event", content = "detail")]
pub enum SocketEvent {
    #[serde(rename_all = "camelCase")]
    Welcome { room_code: String },
    UserJoin(Uuid),
    UserLeave(Uuid),
    Broadcast { from: Uuid, message: String },
    Error(String),
}

impl From<RoomEvent> for SocketEvent {
    fn from(event: RoomEvent) -> Self {
        match event {
            RoomEvent::UserJoin(id) => {
                Self::UserJoin(id)
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
    UserJoin(Uuid),
    UserLeave(Uuid),
    Broadcast { from: Uuid, message: String },
}
