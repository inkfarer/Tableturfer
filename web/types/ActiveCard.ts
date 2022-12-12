import { Position } from '~/types/Position';
import { Card } from '~/types/Card';

export interface ActiveCard extends Card {
    origin: Position
}
