import { MapSquareType } from '~/types/MapSquareType';

export interface GameMap {
    name: string
    squares: MapSquareType[][]
}
