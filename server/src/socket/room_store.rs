use std::collections::HashMap;
use chrono::{DateTime, Utc};
use tokio::sync::broadcast;
use rand::distributions::{Alphanumeric, DistString};
use uuid::Uuid;
use itertools::Itertools;
use serde::Serialize;
use crate::game::map::{DEFAULT_GAME_MAP, GameMap};
use crate::socket::messages::{RoomEvent, SocketError};

const ROOM_CODE_SIZE: usize = 4;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomUser {
    pub joined_at: DateTime<Utc>,
}

pub type RoomSender = broadcast::Sender<RoomEvent>;

#[derive(Clone)]
pub struct Room {
    pub sender: RoomSender,
    pub owner_id: Uuid,
    pub users: HashMap<Uuid, RoomUser>,
    pub map: GameMap,
}

#[derive(Default)]
pub struct SocketRoomStore {
    rooms: HashMap<String, Room>,
}

impl SocketRoomStore {
    pub fn create(&mut self, conn_id: Uuid) -> (String, Room) {
        log::debug!("Connection {conn_id} is creating a new room");
        let mut room_code = Self::generate_room_code();

        while self.rooms.contains_key(&room_code) {
            room_code = Self::generate_room_code();
        }

        let (tx, _rx) = broadcast::channel(100);
        let user = RoomUser {
            joined_at: Utc::now(),
        };
        let room = Room {
            sender: tx.clone(),
            owner_id: conn_id,
            users: HashMap::from([(conn_id, user)]),
            map: DEFAULT_GAME_MAP,
        };

        log::debug!("Connection {conn_id} joins room {room_code}");
        self.rooms.insert(room_code.to_owned(), room.clone());
        (room_code.to_owned(), room)
    }

    // Generates a room code; Determining whether it is unique is up to the caller.
    fn generate_room_code() -> String {
        Alphanumeric.sample_string(&mut rand::thread_rng(), ROOM_CODE_SIZE).to_uppercase()
    }

    pub fn get_and_join_if_exists(&mut self, room_code: &str, conn_id: Uuid) -> Option<Room> {
        log::debug!("Connection {conn_id} attempts to join room {room_code}");
        match self.rooms.get_mut(room_code) {
            Some(room) => {
                let user = RoomUser {
                    joined_at: Utc::now(),
                };
                room.users.insert(conn_id, user.clone());
                room.sender.send(RoomEvent::UserJoin { id: conn_id, user }).ok();

                Some(room.clone())
            }
            None => None
        }
    }

    pub fn remove_user_from_room(&mut self, room_code: &str, conn_id: Uuid) {
        log::debug!("WS connection {conn_id} leaves room {room_code}");
        if let Some(room) = self.rooms.get_mut(room_code) {
            if room.users.remove(&conn_id).is_some() {
                room.sender.send(RoomEvent::UserLeave(conn_id)).ok();

                if room.users.is_empty() {
                    log::debug!("Room {room_code} is now empty, clearing it for reuse");
                    self.rooms.remove(room_code);
                } else if room.owner_id == conn_id {
                    match room.users.clone().into_iter()
                        .sorted_by(|(_id_a, user_a), (_id_b, user_b)| Ord::cmp(&user_a.joined_at, &user_b.joined_at))
                        .next() {
                        Some((first_user_id, _first_user)) => {
                            log::debug!("Room {room_code} is now owned by {first_user_id}");
                            room.owner_id = first_user_id;
                            room.sender.send(RoomEvent::OwnerChange(first_user_id)).ok();
                        }
                        None => {
                            unreachable!("Room has users, but the first user to join could not be found?");
                        }
                    }
                }
            }
        }
    }

    fn is_room_owner(&self, room_code: &str, conn_id: Uuid) -> bool {
        if let Some(room) = self.rooms.get(room_code) {
            room.owner_id == conn_id
        } else {
            false
        }
    }

    pub fn set_map(&mut self, conn_id: Uuid, room_code: &str, map: GameMap) -> Result<(), SocketError> {
        if !self.is_room_owner(room_code, conn_id) {
            return Err(SocketError::UserNotRoomOwner);
        }

        if let Some(room) = self.rooms.get_mut(room_code) {
            room.map = map.clone();
            room.sender.send(RoomEvent::MapChange(map)).ok();
        }

        Ok(())
    }
}

