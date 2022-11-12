use std::sync::Arc;
use tokio::sync::mpsc;
use uuid::Uuid;
use crate::AppState;
use crate::socket::messages::{RoomEvent, SocketEvent};
use crate::socket::room_store::Room;

// Receives messages from a chosen room and forwards them to a websocket channel.
// A client can only listen to one room at a time.
pub struct RoomListener {
    connection_id: Uuid,
    room_sender: Option<Room>,
    state: Arc<AppState>,
    socket_sender: mpsc::Sender<SocketEvent>,
}

impl RoomListener {
    pub fn new(connection_id: Uuid, state: Arc<AppState>, socket_sender: mpsc::Sender<SocketEvent>) -> Self {
        RoomListener {
            connection_id,
            room_sender: None,
            state,
            socket_sender,
        }
    }

    pub async fn join_room(&mut self, room_name: &str) {
        self.leave_room();

        let room_name = room_name.to_owned();
        let socket_tx = self.socket_sender.clone();
        let handler_id = self.connection_id.clone();
        let mut room_store = self.state.room_store.lock().unwrap();
        let room = room_store.get_or_create(&room_name);
        let mut room_rx = room.subscribe();
        self.room_sender = Some(room.clone());

        tokio::spawn(async move {
            room.send(RoomEvent::UserJoin(handler_id)).unwrap();

            while let Ok(msg) = room_rx.recv().await {
                if let RoomEvent::UserLeave(id) = msg {
                    if id == handler_id {
                        log::debug!("{handler_id} has left room {room_name}");
                        break;
                    }
                }

                // If we can't transmit a message to the socket's channel, the socket must be closed
                if socket_tx.send(SocketEvent::from(msg)).await.is_err() {
                    log::debug!("socket_sender for connection {handler_id} is unavailable, no longer listening to room {room_name}");
                    break;
                }
            }
        });
    }

    pub fn leave_room(&self) {
        self.send_event_to_room(RoomEvent::UserLeave(self.connection_id));
    }

    pub fn broadcast(&self, msg: &str) {
        self.send_event_to_room(RoomEvent::Broadcast { from: self.connection_id.clone(), message: msg.to_owned() });
    }

    fn send_event_to_room(&self, event: RoomEvent) {
        if let Some(room) = self.room_sender.clone() {
            room.send(event).unwrap();
        }
    }
}
