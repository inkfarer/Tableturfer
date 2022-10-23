import { MapSquareType } from '~/types/MapSquareType';
import { defineStore } from 'pinia';
import { GameMap } from '~/types/GameMap';

interface GameBoardStore {
    name: string
    board: MapSquareType[][]
}

export const useGameBoardStore = defineStore('gameBoard', {
    state: (): GameBoardStore => ({
        name: 'unknown',
        board: []
    }),
    actions: {
        setBoard(map: GameMap) {
            this.name = map.name;
            this.board = map.squares;
        }
    }
});
