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

impl RoomUser {
    fn new() -> Self {
        RoomUser {
            joined_at: Utc::now(),
        }
    }
}

pub type RoomSender = broadcast::Sender<RoomEvent>;

#[derive(Clone)]
pub struct Room {
    pub sender: RoomSender,
    pub owner_id: Uuid,
    pub opponent_id: Option<Uuid>,
    pub users: HashMap<Uuid, RoomUser>,
    pub map: GameMap,
}

impl Room {
    fn new(owner_id: Uuid) -> Self {
        Room {
            sender: broadcast::channel(100).0,
            owner_id,
            opponent_id: None,
            users: HashMap::from([(owner_id, RoomUser::new())]),
            map: DEFAULT_GAME_MAP,
        }
    }

    fn add_user(&mut self, id: Uuid) {
        let user = RoomUser::new();

        self.users.insert(id, user.clone());
        self.sender.send(RoomEvent::UserJoin { id, user }).ok();

        if self.opponent_id.is_none() {
            self.set_opponent(Some(id));
        }
    }

    fn remove_user(&mut self, id: Uuid) {
        if self.users.remove(&id).is_some() {
            self.sender.send(RoomEvent::UserLeave(id)).ok();

            if !self.users.is_empty() {
                if self.owner_id == id {
                    if let Some((first_user_id, _first_user)) = self.users.clone().into_iter()
                        .sorted_by(|(_id_a, user_a), (_id_b, user_b)| Ord::cmp(&user_a.joined_at, &user_b.joined_at))
                        .next() {
                        self.set_owner(first_user_id);
                    }
                } else if self.is_opponent(id) {
                    let opponent_candidate = self.users.clone().into_iter()
                        .filter(|(id, _user)| id != &self.owner_id)
                        .sorted_by(|(_id_a, user_a), (_id_b, user_b)| Ord::cmp(&user_a.joined_at, &user_b.joined_at))
                        .next();

                    self.set_opponent(opponent_candidate.map_or(None, |(id, _user)| Some(id)));
                }
            }
        }
    }

    fn is_opponent(&self, id: Uuid) -> bool {
        self.opponent_id.is_some() && self.opponent_id.unwrap() == id
    }

    fn set_owner(&mut self, id: Uuid) {
        if self.is_opponent(id) {
            self.set_opponent(None);
        }

        self.owner_id = id;
        self.sender.send(RoomEvent::OwnerChange(id)).ok();
    }

    fn set_opponent(&mut self, id: Option<Uuid>) {
        self.opponent_id = id;
        self.sender.send(RoomEvent::OpponentChange(id)).ok();
    }

    fn set_map(&mut self, map: GameMap) {
        self.map = map.clone();
        self.sender.send(RoomEvent::MapChange(map)).ok();
    }
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

        let room = Room::new(conn_id);

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
                room.add_user(conn_id);

                Some(room.clone())
            }
            None => None
        }
    }

    pub fn remove_user_from_room(&mut self, room_code: &str, conn_id: Uuid) {
        log::debug!("WS connection {conn_id} leaves room {room_code}");
        if let Some(room) = self.rooms.get_mut(room_code) {
            room.remove_user(conn_id);

            if room.users.is_empty() {
                log::debug!("Room {room_code} is now empty, clearing it for reuse");
                self.rooms.remove(room_code);
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
            room.set_map(map);
        }

        Ok(())
    }
}

