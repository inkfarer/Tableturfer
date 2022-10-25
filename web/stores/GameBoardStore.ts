import { MapSquareType } from '~/types/MapSquareType';
import { defineStore } from 'pinia';
import { GameMap } from '~/types/GameMap';
import { findIndex2D, slice2D } from '~/helpers/ArrayHelper';
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
        },
        isPlaceable() {
            return (position: Position, squares: CardSquareType[][]) => {
                if (squares == null || this.board == null) {
                    return false;
                }

                const cardWidth = squares[0].length;
                const cardHeight = squares.length;

                const boardSquaresAtPlacementLocation = slice2D(
                    this.board,
                    position,
                    { x: position.x + cardWidth, y: position.y + cardHeight });

                // If the resulting slice of the board's tiles is not the same size as the card, the card must be outside the board
                if (boardSquaresAtPlacementLocation.length !== cardHeight
                    || boardSquaresAtPlacementLocation[0].length !== cardWidth) {
                    return false;
                }

                // Check if any of the card's squares overlap with existing tiles
                for (let i = 0; i < boardSquaresAtPlacementLocation.length; i++) {
                    const boardRow = boardSquaresAtPlacementLocation[i];
                    const cardRow = squares[i];

                    for (let j = 0; j < boardRow.length; j++) {
                        const cardSquare = cardRow[j];
                        if (cardSquare === CardSquareType.EMPTY) {
                            continue;
                        }

                        const boardSquare = boardRow[j];
                        if (boardSquare !== MapSquareType.EMPTY) {
                            return false;
                        }
                    }
                }

                return true;
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
            if (!this.isPlaceable(position, squares)) {
                console.warn('Skipping card placement as card is in an invalid position');
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
