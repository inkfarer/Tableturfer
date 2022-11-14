import { AnyRoomEvent } from '~/types/socket/RoomEvent';
import { AnyMessage } from '~/types/socket/EventHelper';

export interface SocketActionMap {
    CreateRoom: never
}

export interface SocketUser {
    joinedAt: string
}

export interface SocketMessageMap {
    Error: string
    Welcome: {
        id: string,
        roomCode: string,
        users: Record<string, SocketUser>,
        owner: string
    }
    RoomEvent: AnyRoomEvent
}

export type AnySocketMessage = AnyMessage<SocketMessageMap>;
