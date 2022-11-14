import { SocketUser } from '~/types/socket/SocketEvent';
import { AnyMessage } from '~/types/socket/EventHelper';

export interface RoomEventMap {
    UserJoin: { id: string, user: SocketUser }
    UserLeave: string
    OwnerChange: string
}

export type AnyRoomEvent = AnyMessage<RoomEventMap>;
