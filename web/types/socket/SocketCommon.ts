import { Position } from '~/types/Position';
import { CardRotation } from '~/types/CardRotation';

export interface RoomConfig {
    turnTimerSeconds: number | null
}

export interface PlaceCardMove {
    type: 'PlaceCard'
    cardName: string
    position: Position
    rotation: CardRotation
    special: boolean
}

export interface PassMove {
    type: 'Pass'
    cardName: string
}

export type PlayerMove = PlaceCardMove | PassMove;
