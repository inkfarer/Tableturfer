import { MapSquareType } from '~/types/MapSquareType';
import { defineStore } from 'pinia';
import { GameMap } from '~/types/GameMap';
import { every2D, findIndex2D, slice2D, some2D } from '~/helpers/ArrayHelper';
import { CardSize, useActiveCardStore } from '~/stores/ActiveCardStore';
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
            return (position: Position, cardSquares: CardSquareType[][]) => {
                if (cardSquares == null || this.board == null) {
                    return false;
                }

                const cardWidth = cardSquares[0].length;
                const cardHeight = cardSquares.length;
                const placementCheckSquares = slice2D(
                    this.board,
                    position,
                    { x: position.x + cardWidth - 1, y: position.y + cardHeight - 1 });

                // If the resulting slice of the board's tiles is not the same size as the card, the card must be outside the board
                if (placementCheckSquares.length !== cardHeight
                    || placementCheckSquares[0].length !== cardWidth) {
                    return false;
                }

                const acceptedNearbyBoardSquares = [MapSquareType.FILL_ALPHA, MapSquareType.SPECIAL_ALPHA];

                return every2D(cardSquares, (cardSquare, position) => {
                    // Are any squares outside the map or covering existing squares?
                    const boardSquare = placementCheckSquares[position.y][position.x];
                    return cardSquare === CardSquareType.EMPTY || boardSquare === MapSquareType.EMPTY;
                }) && some2D(cardSquares, (cardSquare, { x, y }) => {
                    // Are there some squares that have existing squares present next to them?
                    if (cardSquare === CardSquareType.EMPTY) {
                        return false;
                    }

                    const boardSquaresAroundCardSquare = slice2D<MapSquareType>(
                        this.board,
                        { x: position.x - 1 + x, y: position.y - 1 + y },
                        { x: position.x + 1 + x, y: position.y + 1 + y });

                    return some2D(boardSquaresAroundCardSquare, square => acceptedNearbyBoardSquares.includes(square));
                });
            };
        },
        boardSquaresUnderCard() {
            return (position: Position, cardSize?: CardSize) => {
                if (cardSize == null) {
                    const activeCardStore = useActiveCardStore();
                    cardSize = activeCardStore.cardSize;
                }

                return slice2D(
                    this.board,
                    position,
                    { x: position.x + cardSize.width - 1, y: position.y + cardSize.height - 1 },
                    MapSquareType.OUT_OF_BOUNDS);
            };
        },
        cardIsOutOfBounds() {
            return (position: Position, cardSize?: CardSize) => {
                if (cardSize == null) {
                    const activeCardStore = useActiveCardStore();
                    cardSize = activeCardStore.cardSize;
                }

                return position.x < 0 || position.y < 0
                    || position.x + cardSize.width > this.boardSize.width
                    || position.y + cardSize.height > this.boardSize.height;
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
