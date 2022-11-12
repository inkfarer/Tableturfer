pub mod room_listener;
pub mod room_store;
pub mod messages;

use std::sync::{Arc};
use axum::extract::{State, WebSocketUpgrade};
use axum::extract::ws::{Message, WebSocket};
use axum::response::IntoResponse;
use futures::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use uuid::Uuid;
use messages::SocketRequest;
use room_listener::RoomListener;
use crate::AppState;
use crate::socket::messages::SocketEvent;

pub async fn socket_handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle(socket, state))
}

async fn handle(stream: WebSocket, state: Arc<AppState>) {
    let id = Uuid::new_v4();
    // Split the stream so we can create two separate tasks for getting data to and from the socket
    let (mut sender, mut receiver) = stream.split();
    // As the socket's stream requires a mutable reference to send messages, we create a new channel
    // here that as many separate threads can send messages into as needed
    let (socket_tx, mut socket_rx) = mpsc::channel(8);

    log::debug!("Starting WS connection {id}");

    let mut receive_from_client_task = tokio::spawn(async move {
        let mut room_handler = RoomListener::new(id.clone(), state, socket_tx.clone());

        // Ignore non-text messages, keep listening until we get an error
        while let Some(Ok(message)) = receiver.next().await {
            match message {
                Message::Text(text) => {
                    match serde_json::from_str(&text) {
                        Ok(action) => {
                            match action {
                                SocketRequest::JoinRoom(room_name) => {
                                    room_handler.join_room(&room_name).await;
                                }
                                SocketRequest::LeaveRoom => {
                                    room_handler.leave_room();
                                }
                                SocketRequest::Broadcast(msg) => {
                                    room_handler.broadcast(&msg);
                                }
                            }
                        }
                        Err(err) => {
                            socket_tx.send(SocketEvent::Error(err.to_string())).await.unwrap();
                        }
                    }
                }
                Message::Close(_) => {
                    room_handler.leave_room();
                }
                _ => {}
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

    // One task completing should abort the other
    tokio::select! {
        _ = (&mut receive_from_client_task) => return_to_client_task.abort(),
        _ = (&mut return_to_client_task) => receive_from_client_task.abort()
    }

    log::debug!("WS connection {id} has shut down");
}
