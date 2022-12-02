import { SocketUser } from '~/types/socket/SocketEvent';
import { AnyMessage } from '~/types/socket/EventHelper';
import { PlayerMove } from '~/types/socket/SocketCommon';
import { PlayerTeam } from '~/types/PlayerTeam';

export interface RoomEventMap {
    UserJoin: { id: string, user: SocketUser }
    UserUpdate: { id: string, user: SocketUser }
    UserLeave: string
    OwnerChange: string
    MapChange: string
    OpponentChange: string | null
    StartGame: never
    MoveReceived: PlayerTeam
    MovesApplied: { [team in PlayerTeam]: PlayerMove }
    HandAssigned: string[]
    NextCardDrawn: { newCard: string, replacing: string }
    ReturnToRoom: never
}

export type AnyRoomEvent = AnyMessage<RoomEventMap>;
