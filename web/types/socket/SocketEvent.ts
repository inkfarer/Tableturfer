import { AnyRoomEvent } from '~/types/socket/RoomEvent';
import { AnyMessage } from '~/types/socket/EventHelper';

export interface SocketErrorMap {
    MessageParsingFailed: never
    UserNotRoomOwner: never
}

export type AnySocketError = {
    [K in keyof SocketErrorMap]: { code: K, detail: SocketErrorMap[K] }
}[keyof SocketErrorMap];

export interface SocketUser {
    joinedAt: string
}

export interface SocketMessageMap {
    Error: AnySocketError
    Welcome: {
        id: string,
        roomCode: string,
        users: Record<string, SocketUser>,
        owner: string,
        opponent: string | null,
        map: string
    }
    RoomEvent: AnyRoomEvent
}

export type AnySocketMessage = AnyMessage<SocketMessageMap>;
