import { PlayerMove } from '~/types/socket/SocketCommon';

export interface SocketActionMap {
    SetMap: string
    StartGame: never
    ProposeMove: PlayerMove
    SetDeck: string[]
    ReturnToRoom: never
}
