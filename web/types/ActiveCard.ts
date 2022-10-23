import { CardRarity } from '~/types/CardRarity';
import { CardSquareType } from '~/types/CardSquareType';
import { Position } from '~/types/Position';

export interface ActiveCard {
    rowId: string
    category: string
    name: string
    number: number
    rarity: CardRarity
    season: number
    specialCost: number
    squares: Array<Array<CardSquareType>>
    origin: Position
}
