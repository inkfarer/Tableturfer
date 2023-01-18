import { PlayerMove } from '~/types/socket/SocketCommon';

export interface SocketActionMap {
    SetMap: string
    StartGame: never
    RequestRedraw: never
    ProposeMove: PlayerMove
    SetDeck: { id: string, cards: string[] }
    ReturnToRoom: never
    Ping: never
}
