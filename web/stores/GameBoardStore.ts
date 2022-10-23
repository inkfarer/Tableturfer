import { MapSquareType } from '~/types/MapSquareType';
import { defineStore } from 'pinia';
import { GameMap } from '~/types/GameMap';
import { findIndex2D } from '~/helpers/ArrayHelper';
import { useActiveCardStore } from '~/stores/ActiveCardStore';

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

            const startSquarePosition = findIndex2D(map.squares, square => square === MapSquareType.SPECIAL_ALPHA);
            if (startSquarePosition != null) {
                const activeCardStore = useActiveCardStore();
                activeCardStore.setPosition(startSquarePosition);
            }
        }
    }
});
