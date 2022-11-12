use std::collections::HashMap;
use tokio::sync::broadcast;
use rand::distributions::{Alphanumeric, DistString};
use crate::socket::messages::RoomEvent;

const ROOM_CODE_SIZE: usize = 4;
pub type Room = broadcast::Sender<RoomEvent>;

#[derive(Default)]
pub struct SocketRoomStore {
    rooms: HashMap<String, Room>,
}

impl SocketRoomStore {
    pub fn create(&mut self) -> (String, Room) {
        let mut room_code = Self::generate_room_code();

        while self.rooms.contains_key(&room_code) {
            room_code = Self::generate_room_code();
        }

        let (tx, _rx) = broadcast::channel(100);
        self.rooms.insert(room_code.to_owned(), tx.clone());
        (room_code.to_owned(), tx)
    }

    // Generates a room code; Determining whether it is unique is up to the caller.
    fn generate_room_code() -> String {
        Alphanumeric.sample_string(&mut rand::thread_rng(), ROOM_CODE_SIZE).to_uppercase()
    }

    pub fn get(&self, room_code: &str) -> Option<Room> {
        match self.rooms.get(room_code) {
            Some(room) => Some(room.clone()),
            None => None
        }
    }
}

