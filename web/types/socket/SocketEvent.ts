import { AnyRoomEvent } from '~/types/socket/RoomEvent';
import { AnyError, AnyMessage } from '~/types/socket/EventHelper';

interface InvalidMoveErrorMap {
    CardNotFound: never
    CardNotInHand: never
    CannotAffordSpecial: never
    CardOutOfBounds: never
    CardOnDisallowedSquares: never
    NoExpectedSquaresNearCard: never
}

export interface GameErrorMap {
    InvalidMove: AnyError<InvalidMoveErrorMap>
    CardNotFound: never
    MapNotFound: never
    IncorrectDeckSize: never
    GameEnded: never
}

export type AnyGameError = AnyError<GameErrorMap>;

export interface SocketErrorMap {
    MessageParsingFailed: never
    UserNotRoomOwner: never
    UserNotPlaying: never
    RoomNotFound: string
    MissingOpponent: never
    RoomStarted: never
    RoomNotStarted: never
    DecksNotChosen: never
    GameError: AnyGameError
}

export type AnySocketError = AnyError<SocketErrorMap>;

export interface SocketUserDeck {
    id: string
    cards: string[]
}

export interface SocketUser {
    username: string
    joinedAt: string
    deck: SocketUserDeck | null
}

export interface SocketMessageMap {
    Error: AnySocketError
    Welcome: {
        id: string
        roomCode: string
        users: Record<string, SocketUser>
        owner: string
        opponent: string | null
        map: string
        started: boolean
    }
    RoomEvent: AnyRoomEvent
}

export type AnySocketMessage = AnyMessage<SocketMessageMap>;
