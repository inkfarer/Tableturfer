import { SocketUser } from '~/types/socket/SocketEvent';
import { AnyMessage } from '~/types/socket/EventHelper';

export interface RoomEventMap {
    UserJoin: { id: string, user: SocketUser }
    UserLeave: string
    OwnerChange: string
    MapChange: string
    OpponentChange: string | null
    StartGame: never
}

export type AnyRoomEvent = AnyMessage<RoomEventMap>;
