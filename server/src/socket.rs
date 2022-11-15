pub mod room_store;
pub mod messages;
mod close_code;

use std::sync::{Arc};
use axum::extract::{Query, State, WebSocketUpgrade};
use axum::extract::ws::{Message, WebSocket};
use axum::response::IntoResponse;
use futures::{SinkExt, StreamExt};
use serde::Deserialize;
use tokio::sync::mpsc;
use uuid::Uuid;
use messages::SocketRequest;
use crate::AppState;
use crate::socket::close_code::SocketCloseCode;
use crate::socket::messages::{RoomEvent, SocketError, SocketEvent};

#[derive(Debug, Deserialize)]
pub struct SocketRouteParams {
    room: Option<String>
}

pub async fn socket_handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>, Query(params): Query<SocketRouteParams>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle(socket, state, params.room))
}

async fn handle(stream: WebSocket, state: Arc<AppState>, room_code: Option<String>) {
    let id = Uuid::new_v4();

    let (room_code, room) = {
        let mut room_store = state.room_store.write().unwrap();

        match room_code {
            Some(room_code) => {
                let room_code = room_code.to_uppercase();
                (room_code.to_owned(), room_store.get_and_join_if_exists(&room_code, id))
            }
            None => {
                let (room_code, room) = room_store.create(id.clone());
                (room_code, Some(room))
            }
        }
    };

    // Split the stream so we can create two separate tasks for getting data to and from the socket
    let (mut sender, mut receiver) = stream.split();

    if room.is_none() {
        log::debug!("Rejecting WS connection as it attempted to join a non-existent room");
        sender.send(Message::Close(Some(SocketCloseCode::RoomNotFound(room_code).into()))).await.unwrap();
        return;
    }

    let room = room.unwrap();
    // As the socket's stream requires a mutable reference to send messages, we create a new channel
    // here that as many separate threads can send messages into as needed
    let (socket_tx, mut socket_rx) = mpsc::channel(8);

    log::debug!("Starting WS connection {id}");

    let room_tx = room.sender.clone();
    // Subscribe to events from the room before we send any events in, otherwise we may encounter errors
    let mut room_rx = room_tx.subscribe();
    socket_tx.send(SocketEvent::Welcome {
        id,
        room_code: room_code.to_owned(),
        users: room.users,
        owner: room.owner_id,
        opponent: room.opponent_id,
        map: room.map,
    }).await.unwrap();

    let room_tx_from_client = room_tx.clone();
    let socket_tx_from_client = socket_tx.clone();
    let state_from_client = state.clone();
    let room_code_from_client = room_code.clone();
    let mut receive_from_client_task = tokio::spawn(async move {
        // Ignore non-text messages, keep listening until we get an error
        while let Some(Ok(message)) = receiver.next().await {
            if let Message::Text(text) = message {
                match serde_json::from_str(&text) {
                    Ok(action) => {
                        match action {
                            SocketRequest::Broadcast(msg) => {
                                room_tx_from_client.send(RoomEvent::Broadcast { from: id, message: msg.to_owned() }).unwrap();
                            }
                            SocketRequest::SetMap(map) => {
                                let action_result = {
                                    let mut room_store = state_from_client.room_store.write().unwrap();
                                    room_store.set_map(id, &room_code_from_client, map)
                                };

                                if let Err(err) = action_result {
                                    socket_tx_from_client.send(SocketEvent::Error(err)).await.unwrap();
                                }
                            }
                        }
                    }
                    Err(_) => {
                        socket_tx_from_client.send(SocketEvent::Error(SocketError::MessageParsingFailed)).await.unwrap();
                    }
                }
            }
        }
    });

    let receive_from_room_task = tokio::spawn(async move {
        while let Ok(msg) = room_rx.recv().await {
            if socket_tx.send(SocketEvent::RoomEvent(msg)).await.is_err() {
                break;
            }
        }
    });

    let mut return_to_client_task = tokio::spawn(async move {
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
    });

    // One task completing should abort the others
    tokio::select! {
        _ = (&mut receive_from_client_task) => return_to_client_task.abort(),
        _ = (&mut return_to_client_task) => receive_from_client_task.abort()
    }
    receive_from_room_task.abort();
    {
        let mut room_store = state.room_store.write().unwrap();
        room_store.remove_user_from_room(&room_code, id);
    }

    log::debug!("WS connection {id} has shut down");
}
