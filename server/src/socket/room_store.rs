use std::borrow::BorrowMut;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use tokio::sync::broadcast;
use rand::distributions::{Alphanumeric, DistString};
use uuid::Uuid;
use itertools::Itertools;
use serde::Serialize;
use crate::game::map::{DEFAULT_GAME_MAP, GameMap};
use crate::game::state::{GameError, GameState, PlayerMove};
use crate::game::team::PlayerTeam;
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
    pub game_state: Option<GameState>,
}

impl Room {
    fn new(owner_id: Uuid) -> Self {
        Room {
            sender: broadcast::channel(100).0,
            owner_id,
            opponent_id: None,
            users: HashMap::from([(owner_id, RoomUser::new())]),
            map: DEFAULT_GAME_MAP,
            game_state: None,
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

            if !self.users.is_empty() && !self.game_started() {
                if self.owner_id == id {
                    // todo: find a new opponent if the first user was previously the opponent
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

    pub fn is_opponent(&self, id: Uuid) -> bool {
        self.opponent_id.is_some() && self.opponent_id.unwrap() == id
    }

    fn set_owner(&mut self, id: Uuid) {
        if !self.game_started() {
            if self.is_opponent(id) {
                self.set_opponent(None);
            }

            self.owner_id = id;
            self.sender.send(RoomEvent::OwnerChange(id)).ok();
        }
    }

    fn set_opponent(&mut self, id: Option<Uuid>) {
        if !self.game_started() {
            self.opponent_id = id;
            self.sender.send(RoomEvent::OpponentChange(id)).ok();
        }
    }

    pub fn set_map(&mut self, map: GameMap) -> Result<(), SocketError> {
        if !self.game_started() {
            self.map = map.clone();
            self.sender.send(RoomEvent::MapChange(map)).ok();
            Ok(())
        } else {
            Err(SocketError::RoomStarted)
        }
    }

    pub fn start_game(&mut self) -> Result<(), SocketError> {
        if self.opponent_id.is_none() {
            Err(SocketError::MissingOpponent)
        } else {
            self.game_state = Some(GameState::new(self.map.to_squares()));
            self.sender.send(RoomEvent::StartGame).ok();
            Ok(())
        }
    }

    pub fn propose_move(&mut self, team: PlayerTeam, player_move: PlayerMove) -> Result<(), SocketError> {
        let sender = self.sender.clone();
        self.do_with_game(|game| {
            let result = game.propose_move(team.clone(), player_move);
            if result.is_ok() {
                sender.send(RoomEvent::MoveReceived(team)).ok();
                if game.all_players_have_moved() {
                    let moves = game.apply_moves();
                    sender.send(RoomEvent::MovesApplied(moves)).ok();
                }
            }
            result
        })
    }

    fn do_with_game<F>(&mut self, action: F) -> Result<(), SocketError>
        where
            F: FnOnce(&mut GameState) -> Result<(), GameError>,
    {
        if let Some(game) = self.game_state.borrow_mut() {
            action(game).map_err(|e| SocketError::GameError(e))
        } else {
            Err(SocketError::RoomNotStarted)
        }
    }

    pub fn game_started(&self) -> bool {
        self.game_state.is_some()
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
                // todo: currently, if the room owner is alone in a room and refreshes their browser, they'll receive a "room not found" error
                log::debug!("Room {room_code} is now empty, clearing it for reuse");
                self.rooms.remove(room_code);
            }
        }
    }

    pub fn get_mut(&mut self, room_code: &str) -> Option<&mut Room> {
        self.rooms.get_mut(room_code)
    }
}

