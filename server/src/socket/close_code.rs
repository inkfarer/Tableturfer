use std::borrow::Cow;
use axum::extract::ws::CloseFrame;

pub enum SocketCloseCode {
    RoomNotFound(String),
    InvalidUsername,
}

impl Into<CloseFrame<'_>> for SocketCloseCode {
    fn into(self) -> CloseFrame<'static> {
        match self {
            SocketCloseCode::RoomNotFound(room_code) => {
                CloseFrame {
                    code: 4000,
                    reason: Cow::from(format!("Could not find room \"{room_code}\""))
                }
            },
            SocketCloseCode::InvalidUsername => {
                CloseFrame {
                    code: 4001,
                    reason: Cow::from("Either no username or an invalid username was supplied.")
                }
            }
        }
    }
}
