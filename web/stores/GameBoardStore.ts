import { MapSquareType } from '~/types/MapSquareType';
import { defineStore } from 'pinia';
import { GameMap } from '~/types/GameMap';
import { findIndex2D } from '~/helpers/ArrayHelper';
import { useActiveCardStore } from '~/stores/ActiveCardStore';
import { Position } from '~/types/Position';
import { CardSquareType } from '~/types/CardSquareType';
import cloneDeep from 'lodash/cloneDeep';

interface GameBoardStore {
    name: string
    board: MapSquareType[][] | null
}

export const useGameBoardStore = defineStore('gameBoard', {
    state: (): GameBoardStore => ({
        name: 'unknown',
        board: null
    }),
    getters: {
        boardSize: state => {
            if (state.board == null) {
                return { width: 0, height: 0 };
            }

            return {
                width: state.board[0].length,
                height: state.board.length
            };
        }
    },
    actions: {
        setBoard(map: GameMap) {
            this.name = map.name;
            this.board = map.squares;

            const startSquarePosition = findIndex2D(map.squares, square => square === MapSquareType.SPECIAL_ALPHA);
            if (startSquarePosition != null) {
                const activeCardStore = useActiveCardStore();
                activeCardStore.setPosition(startSquarePosition);
            }
        },
        placeCard(position: Position, squares: CardSquareType[][]) {
            const cardWidth = squares[0].length;
            const cardHeight = squares.length;
            if (position.x < 0 || position.y < 0
                || position.x + cardWidth > this.boardSize.width
                || position.y + cardHeight > this.boardSize.height) {
                console.warn('Skipping card placement as card is out of bounds');
                return;
            }

            const newBoard = cloneDeep(this.board);
            squares.forEach((row, rowIndex) => {
                row.forEach((square, colIndex) => {
                    switch (square) {
                        case CardSquareType.EMPTY:
                            break;
                        case CardSquareType.FILL:
                            newBoard[position.y + rowIndex][position.x + colIndex] = MapSquareType.FILL_ALPHA;
                            break;
                        case CardSquareType.SPECIAL:
                            newBoard[position.y + rowIndex][position.x + colIndex] = MapSquareType.SPECIAL_ALPHA;
                            break;
                    }
                });
            });
            this.board = newBoard;
        }
    }
});
