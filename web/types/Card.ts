import { CardRarity } from './CardRarity';
import { CardSquareType } from './CardSquareType';

export interface Card {
    rowId: string
    category: string
    name: string
    number: number
    rarity: CardRarity
    season: number
    specialCost: number
    squares: Array<CardSquareType>
}
