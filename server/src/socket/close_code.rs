use std::borrow::Cow;
use axum::extract::ws::CloseFrame;

pub enum SocketCloseCode {
    RoomNotFound(String)
}

impl Into<CloseFrame<'_>> for SocketCloseCode {
    fn into(self) -> CloseFrame<'static> {
        match self {
            SocketCloseCode::RoomNotFound(room_code) => {
                CloseFrame {
                    code: 4000,
                    reason: Cow::from(format!("Could not find room \"{room_code}\""))
                }
            }
        }
    }
}
