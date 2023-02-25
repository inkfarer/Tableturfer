pub mod room_store;
pub mod messages;
mod close_code;
mod action_handler;

use std::sync::{Arc};
use axum::extract::{Query, State, WebSocketUpgrade};
use axum::extract::ws::{Message, WebSocket};
use axum::response::IntoResponse;
use futures::{SinkExt, StreamExt};
use futures::stream::{SplitSink, SplitStream};
use serde::Deserialize;
use tokio::sync::{broadcast, mpsc};
use tokio::task::JoinHandle;
use uuid::Uuid;
use crate::AppState;
use crate::socket::action_handler::SocketActionHandler;
use crate::socket::close_code::SocketCloseCode;
use crate::socket::messages::{RoomEvent, SocketError, SocketEvent};
use crate::socket::room_store::Room;

#[derive(Debug, Deserialize)]
pub struct SocketRouteParams {
    room: Option<String>,
    username: Option<String>,
}

pub type SocketSender = mpsc::Sender<SocketEvent>;

pub struct SocketHandler {
    id: Uuid,
    socket_channel: SocketSender,
    room_channel: broadcast::Sender<RoomEvent>,
    state: Arc<AppState>,
    room_code: String,
}

impl SocketHandler {
    pub async fn request_handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>, Query(params): Query<SocketRouteParams>) -> impl IntoResponse {
        ws.on_upgrade(move |socket| Self::try_init(socket, params, state))
    }

    fn username_is_valid(username: Option<String>) -> bool {
        if username.is_none() {
            return false
        }

        let username = username.unwrap();
        !username.is_empty() && username.len() <= 25
    }

    async fn try_init(socket: WebSocket, query: SocketRouteParams, state: Arc<AppState>) {
        // Split the stream so we can create two separate tasks for getting data to and from the socket
        let (mut sender, receiver) = socket.split();
        let id = Uuid::new_v4();
        // As the socket's stream requires a mutable reference to send messages, we create a new channel
        // here that as many separate threads can send messages into as needed
        let socket_channel = mpsc::channel(8);

        if !Self::username_is_valid(query.username.clone()) {
            log::debug!("Rejecting WS connection as for having an invalid username");
            sender.send(Message::Close(Some(SocketCloseCode::InvalidUsername.into()))).await.unwrap();
            return;
        }

        let username = query.username.unwrap();

        let (room_code, room) = Self::get_and_join_room(id, &username, state.clone(), query.room, socket_channel.0.clone()).await;
        if room.is_none() {
            log::debug!("Rejecting WS connection as it attempted to join a non-existent room");
            sender.send(Message::Close(Some(SocketCloseCode::RoomNotFound(room_code).into()))).await.unwrap();
            return;
        }

        let room = room.unwrap();

        Self {
            id,
            socket_channel: socket_channel.0,
            room_channel: room.sender.clone(),
            state,
            room_code
        }.init(room, sender, receiver, socket_channel.1).await;
    }

    async fn get_and_join_room(
        id: Uuid,
        username: &str,
        state: Arc<AppState>,
        room_code: Option<String>,
        event_sender: SocketSender
    ) -> (String, Option<Room>)
    {
        let mut room_store = state.room_store.write().await;

        match room_code {
            Some(room_code) => {
                let room_code = room_code.to_uppercase();
                (room_code.to_owned(), room_store.get_and_join_if_exists(&room_code, id, username, event_sender))
            }
            None => {
                let (room_code, room) = room_store.create(id.clone(), username, event_sender);
                (room_code, Some(room))
            }
        }
    }

    async fn init(self, room: Room, sender: SplitSink<WebSocket, Message>, receiver: SplitStream<WebSocket>, socket_channel_receiver: mpsc::Receiver<SocketEvent>) {
        log::debug!("Starting WS connection {}", self.id);

        let mut receive_from_client_task = self.listen_to_client(receiver);
        let receive_from_room_task = self.receive_from_room();
        let mut return_to_client_task = Self::return_to_client(socket_channel_receiver, sender);

        self.socket_channel.send(self.create_welcome_event(room)).await.unwrap();

        // One task completing should abort the others
        tokio::select! {
            _ = (&mut receive_from_client_task) => return_to_client_task.abort(),
            _ = (&mut return_to_client_task) => receive_from_client_task.abort()
        }
        receive_from_room_task.abort();
        {
            let mut room_store = self.state.room_store.write().await;
            room_store.remove_user_from_room(&self.room_code, self.id);
        }

        log::debug!("WS connection {} has shut down", self.id);
    }

    fn create_welcome_event(&self, room: Room) -> SocketEvent {
        SocketEvent::Welcome {
            id: self.id,
            room_code: self.room_code.clone(),
            started: room.game_started(),
            users: room.users,
            owner: room.owner_id,
            opponent: room.opponent_id,
            map: room.map,
            config: room.config,
        }
    }

    fn listen_to_client(&self, mut receiver: SplitStream<WebSocket>) -> JoinHandle<()> {
        let action_handler = SocketActionHandler::new(
            self.id.clone(),
            self.socket_channel.clone(),
            self.state.clone(),
            self.room_code.clone());

        tokio::spawn(async move {
            // Ignore non-text messages, keep listening until we get an error
            while let Some(Ok(message)) = receiver.next().await {
                if let Message::Text(text) = message {
                    match serde_json::from_str(&text) {
                        Ok(action) => action_handler.handle_action(action).await,
                        Err(_) => {
                            log::debug!("Failed to parse message from client");
                            action_handler.send_error(SocketError::MessageParsingFailed).await.unwrap();
                        }
                    }
                }
            }
        })
    }

    fn receive_from_room(&self) -> JoinHandle<()> {
        let mut room_rx = self.room_channel.subscribe();
        let socket_tx = self.socket_channel.clone();

        tokio::spawn(async move {
            while let Ok(msg) = room_rx.recv().await {
                if socket_tx.send(SocketEvent::RoomEvent(msg)).await.is_err() {
                    break;
                }
            }
        })
    }

    fn return_to_client(mut socket_rx: mpsc::Receiver<SocketEvent>, mut sender: SplitSink<WebSocket, Message>) -> JoinHandle<()> {
        tokio::spawn(async move {
            while let Some(msg) = socket_rx.recv().await {
                match serde_json::to_string(&msg) {
                    Ok(msg_string) => {
                        if sender.send(Message::Text(msg_string)).await.is_err() {
                            socket_rx.close();
                            break;
                        }
                    }
                    Err(err) => {
                        log::error!("Failed to serialize message for WS client: {err}");
                    }
                }
            }
        })
    }
}
