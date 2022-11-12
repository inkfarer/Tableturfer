use std::collections::HashMap;
use tokio::sync::broadcast;
use crate::socket::messages::RoomEvent;

pub type Room = broadcast::Sender<RoomEvent>;

#[derive(Default)]
pub struct SocketRoomStore {
    rooms: HashMap<String, Room>,
}

impl SocketRoomStore {
    pub fn get_or_create(&mut self, room_name: &str) -> Room {
        if let Some(room) = self.rooms.get(room_name) {
            return room.clone();
        }

        let (tx, _rx) = broadcast::channel(100);
        self.rooms.insert(room_name.to_owned(), tx.clone());
        tx
    }
}

