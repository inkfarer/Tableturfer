import { SocketUser } from '~/types/socket/SocketEvent';
import { AnyMessage } from '~/types/socket/EventHelper';
import { PlayerMove } from '~/types/socket/SocketCommon';
import { PlayerTeam, TeamMap } from '~/types/PlayerTeam';

export interface RoomEventMap {
    UserJoin: { id: string, user: SocketUser }
    UserUpdate: { id: string, user: SocketUser }
    UserLeave: string
    OwnerChange: string
    MapChange: string
    OpponentChange: string | null
    StartGame: { score: TeamMap<number> }
    MoveReceived: { team: PlayerTeam, remainingTurns: number }
    MovesApplied: { moves: TeamMap<PlayerMove>, score: TeamMap<number> }
    HandAssigned: string[]
    NextCardDrawn: { newCard: string, replacing: string }
    EndGame: { score: TeamMap<number> }
    ReturnToRoom: never
}

export type AnyRoomEvent = AnyMessage<RoomEventMap>;
