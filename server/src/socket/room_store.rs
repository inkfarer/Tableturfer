use std::collections::HashMap;
use std::sync::Arc;
use chrono::{DateTime, Utc};
use indexmap::IndexSet;
use tokio::sync::broadcast;
use rand::distributions::{Alphanumeric, DistString};
use uuid::Uuid;
use itertools::Itertools;
use serde::Serialize;
use crate::game::card::{CardProvider, CardSquareProviderImpl};
use crate::game::map::{DEFAULT_GAME_MAP, MapProvider, MapProviderImpl};
use crate::game::move_validator::MoveValidatorImpl;
use crate::game::state::{DECK_SIZE, GameError, GameState, PlayerMove};
use crate::game::team::PlayerTeam;
use crate::socket::messages::{RoomEvent, SocketError, SocketEvent};
use crate::socket::SocketSender;

const ROOM_CODE_SIZE: usize = 4;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomUser {
    pub joined_at: DateTime<Utc>,
    pub deck: Option<IndexSet<String>>
}

impl RoomUser {
    fn new() -> Self {
        RoomUser {
            joined_at: Utc::now(),
            deck: None,
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
    pub user_channels: HashMap<Uuid, SocketSender>,
    pub map: String,
    pub game_state: Option<GameState>,
    pub card_provider: Arc<dyn CardProvider + Send + Sync>,
    pub map_provider: Arc<dyn MapProvider + Send + Sync>,
}

impl Room {
    fn new(owner_id: Uuid, owner_channel: SocketSender) -> Self {
        Room {
            sender: broadcast::channel(100).0,
            owner_id,
            opponent_id: None,
            users: HashMap::from([(owner_id, RoomUser::new())]),
            user_channels: HashMap::from([(owner_id, owner_channel)]),
            map: DEFAULT_GAME_MAP.to_string(),
            game_state: None,
            card_provider: Arc::new(CardSquareProviderImpl::new()),
            map_provider: Arc::new(MapProviderImpl::new()),
        }
    }

    fn add_user(&mut self, id: Uuid, channel: SocketSender) {
        let user = RoomUser::new();

        self.users.insert(id, user.clone());
        self.user_channels.insert(id, channel);
        self.sender.send(RoomEvent::UserJoin { id, user }).ok();

        if self.opponent_id.is_none() {
            self.set_opponent(Some(id));
        }
    }

    fn remove_user(&mut self, id: Uuid) {
        self.user_channels.remove(&id);

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

    pub fn set_map(&mut self, map: String) -> Result<(), SocketError> {
        if !self.map_provider.exists(&map) {
            return Err(SocketError::GameError(GameError::MapNotFound));
        }

        if !self.game_started() {
            self.map = map.clone();
            self.sender.send(RoomEvent::MapChange(map)).ok();
            Ok(())
        } else {
            Err(SocketError::RoomStarted)
        }
    }

    pub async fn start_game(&mut self) -> Result<(), SocketError> {
        if self.opponent_id.is_none() {
            Err(SocketError::MissingOpponent)
        } else if [self.owner_id, self.opponent_id.unwrap()].iter().any(|user| self.users[user].deck.is_none()) {
            Err(SocketError::DecksNotChosen)
        } else {
            let players = self.get_players();
            let map = self.map_provider.get(&self.map).unwrap();
            let mut game_state = GameState::new(
                map.squares,
                self.card_provider.clone(),
                Arc::new(MoveValidatorImpl::new(self.card_provider.clone())),
                players.into_iter().map(|(team, player)| {
                    (team, player.deck.as_ref().unwrap().clone())
                }).collect()
            );
            self.sender.send(RoomEvent::StartGame).ok();

            let initial_hands = game_state.assign_initial_hands();
            for (team, hand) in initial_hands {
                self.send_to_player(team, SocketEvent::RoomEvent(RoomEvent::HandAssigned(hand))).await;
            }

            self.game_state = Some(game_state);
            Ok(())
        }
    }

    fn get_players(&self) -> HashMap<PlayerTeam, &RoomUser> {
        let mut result = HashMap::from([(PlayerTeam::Alpha, &self.users[&self.owner_id])]);

        if let Some(opponent_id) = self.opponent_id {
            result.insert(PlayerTeam::Bravo, &self.users[&opponent_id]);
        }

        result
    }

    pub fn set_deck(&mut self, id: Uuid, deck: IndexSet<String>) -> Result<(), SocketError> {
        if self.game_started() {
            Err(SocketError::RoomStarted)
        } else if deck.len() != DECK_SIZE {
            Err(SocketError::GameError(GameError::IncorrectDeckSize))
        } else if deck.iter().any(|card| !self.card_provider.exists(card)){
            Err(SocketError::GameError(GameError::CardNotFound))
        } else {
            self.modify_user(id, |user| {
                user.deck = Some(deck);
            });
            Ok(())
        }
    }

    pub async fn propose_move(&mut self, team: PlayerTeam, player_move: PlayerMove) -> Result<(), SocketError> {
        if self.game_state.is_none() {
            return Err(SocketError::RoomNotStarted);
        }

        let game = self.game_state.as_mut().unwrap();
        let sender = self.sender.clone();
        let result = game.propose_move(team.clone(), player_move);
        if result.is_ok() {
            sender.send(RoomEvent::MoveReceived(team)).ok();

            if game.all_players_have_moved() {
                let moves = game.apply_moves();

                sender.send(RoomEvent::MovesApplied(moves.applied_moves.clone())).ok();

                if game.completed() {
                    sender.send(RoomEvent::EndGame { score: game.score() }).ok();
                }

                for (team, next_card) in moves.next_cards {
                    self.send_to_player(
                        team.clone(),
                        SocketEvent::RoomEvent(RoomEvent::NextCardDrawn {
                            new_card: next_card,
                            replacing: moves.applied_moves[&team].card_name().to_owned(),
                        })
                    ).await;
                }
            }
        }
        result.map_err(|e| SocketError::GameError(e))
    }

    pub fn return_to_room(&mut self) {
        self.game_state = None;
        self.sender.send(RoomEvent::ReturnToRoom).ok();
    }

    async fn send_to_player(&self, team: PlayerTeam, message: SocketEvent) {
        let sender: Option<&SocketSender> = match team {
            PlayerTeam::Alpha => self.user_channels.get(&self.owner_id),
            PlayerTeam::Bravo => self.opponent_id.map_or(None, |id| self.user_channels.get(&id)),
        };

        if let Some(sender) = sender {
            sender.send(message).await.ok();
        }
    }

    fn modify_user<F>(&mut self, id: Uuid, action: F) where F: FnOnce(&mut RoomUser) -> () {
        if let Some(user) = self.users.get_mut(&id) {
            action(user);
            self.sender.send(RoomEvent::UserUpdate { id, user: user.clone() }).ok();
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
    pub fn create(&mut self, conn_id: Uuid, conn_channel: SocketSender) -> (String, Room) {
        log::debug!("Connection {conn_id} is creating a new room");
        let mut room_code = Self::generate_room_code();

        while self.rooms.contains_key(&room_code) {
            room_code = Self::generate_room_code();
        }

        let room = Room::new(conn_id, conn_channel);

        log::debug!("Connection {conn_id} joins room {room_code}");
        self.rooms.insert(room_code.to_owned(), room.clone());
        (room_code.to_owned(), room)
    }

    // Generates a room code; Determining whether it is unique is up to the caller.
    fn generate_room_code() -> String {
        Alphanumeric.sample_string(&mut rand::thread_rng(), ROOM_CODE_SIZE).to_uppercase()
    }

    pub fn get_and_join_if_exists(&mut self, room_code: &str, conn_id: Uuid, conn_channel: SocketSender) -> Option<Room> {
        log::debug!("Connection {conn_id} attempts to join room {room_code}");
        match self.rooms.get_mut(room_code) {
            Some(room) => {
                room.add_user(conn_id, conn_channel);

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

