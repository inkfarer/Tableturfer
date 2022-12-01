import { CardRarity } from './CardRarity';
import { CardSquareType } from './CardSquareType';

export interface Card {
    category: string
    name: string
    number: number
    rarity: CardRarity
    season: number
    specialCost: number
    squares: Array<Array<CardSquareType>>
}
