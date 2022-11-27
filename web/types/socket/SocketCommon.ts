import { Position } from '~/types/Position';
import { CardRotation } from '~/types/CardRotation';

export interface PlayerMove {
    cardName: string
    position: Position
    rotation: CardRotation
    special: boolean
}
