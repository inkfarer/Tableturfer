import { Card } from '~/types/Card';
import { Position } from '~/types/Position';
import { ActiveCard } from '~/types/ActiveCard';
import Constants from '~/data/Constants';
import { CardSquareType } from '~/types/CardSquareType';
import chunk from 'lodash/chunk';
import { CardRotation } from '~/types/CardRotation';
import { rotateClockwise, rotateCounterclockwise, some2D } from '~/helpers/ArrayHelper';
import { defineStore } from 'pinia';
import { useGameBoardStore } from '~/stores/GameBoardStore';
import { MapSquareType } from '~/types/MapSquareType';

interface ActiveCardStore {
    activeCard: ActiveCard | null
    internalPosition: Position
    rotation: CardRotation
}

export interface CardSize {
    height: number
    width: number
}

// Attempt to nudge the card back into bounds if required
// Likely quite inefficient but acceptable for now
function withinBoardBounds(position: Position, squares: CardSquareType[][]): Position {
    const cardWidth = squares[0]?.length ?? 0;
    const cardHeight = squares.length;
    const newPosition = { ...position };
    const gameBoardStore = useGameBoardStore();
    const boardSize = gameBoardStore.boardSize;
    const activeCardStore = useActiveCardStore();

    if (
        (cardHeight === 0 && cardWidth === 0)
        || !gameBoardStore.cardIsOutOfBounds(position, { width: cardWidth, height: cardHeight })
    ) {
        return newPosition;
    }

    const someSquaresWithinBounds = (): boolean => {
        const squaresUnderPosition = gameBoardStore.boardSquaresUnderCard(
            activeCardStore.withRotationOffset(newPosition),
            { width: cardWidth, height: cardHeight });
        return some2D(squaresUnderPosition, (boardSquare, position) => {
            const cardSquare = squares[position.y][position.x];

            return cardSquare !== CardSquareType.EMPTY && boardSquare !== MapSquareType.OUT_OF_BOUNDS;
        });
    };

    while (!someSquaresWithinBounds()) {
        if (newPosition.y < 0) {
            newPosition.y++;
        } else if (newPosition.y + cardHeight - 1 >= boardSize.height) {
            newPosition.y--;
        }

        if (!someSquaresWithinBounds()) {
            if (newPosition.x < 0) {
                newPosition.x++;
            } else if (newPosition.x + cardWidth - 1 >= boardSize.width) {
                newPosition.x--;
            }
        } else {
            break;
        }
    }

    return newPosition;
}

export const useActiveCardStore = defineStore('activeCard', {
    state: (): ActiveCardStore => ({
        activeCard: null,
        internalPosition: {
            x: 0,
            y: 0
        },
        rotation: 0
    }),
    getters: {
        cardSize: (state): CardSize => {
            if (state.activeCard == null) {
                return {
                    width: 0,
                    height: 0
                };
            }

            return ({
                width: state.rotation === 0 || state.rotation === 180
                    ? state.activeCard.squares[0].length
                    : state.activeCard.squares.length,
                height: state.rotation === 90 || state.rotation === 270
                    ? state.activeCard.squares[0].length
                    : state.activeCard.squares.length
            });
        },
        position() {
            return this.withRotationOffset(this.internalPosition);
        },
        withRotationOffset() {
            return (position: Position) => {
                return {
                    x: position.x + this.rotationOffset.x,
                    y: position.y + this.rotationOffset.y
                };
            };
        },
        // Strictly matches the way the card moves as it is rotated in-game. Can probably be cleaned up.
        rotationOffset() {
            if (this.rotation === 0) {
                return { x: 0, y: 0 };
            } else {
                const { width, height } = this.cardSize;
                if (width === height) {
                    return { x: 0, y: 0 };
                }

                switch (this.rotation) {
                    case 90: {
                        let x = Math.ceil((width - height) / 2);
                        const y = Math.ceil((height - width) / 2);

                        if (height % 2 === 1 && width % 2 === 0) {
                            x -= 1;
                        }

                        return { x, y };
                    }
                    case 180: {
                        if (height % 2 === 0 && width % 2 === 1) {
                            return { x: 0, y: (height + width) % 2 };
                        } else {
                            return { x: ((height + width) % 2) * -1, y: 0 };
                        }
                    }
                    case 270: {
                        const x = Math.floor((width - height) / 2);
                        let y = x * -1;

                        if (height % 2 === 1 && width % 2 === 0) {
                            y -= 1;
                        }

                        return { x, y };
                    }
                }
            }
        }
    },
    actions: {
        setActiveCard(card: Card | null) {
            // Returns the new origin - Could be avoided later when cards are normalized before they are sent into this function
            const updatePosition = (squares: CardSquareType[][]): Position => {
                const oldOrigin = this.activeCard?.origin ?? { x: 0, y: 0 };

                const cardWidth = squares[0]?.length ?? 0;
                const cardHeight = squares.length;
                const newOrigin = card == null
                    ? { x: 0, y: 0 }
                    : {
                        x: Math.ceil(cardWidth / 2 - 1),
                        y: Math.floor(cardHeight / 2)
                    };

                this.internalPosition = withinBoardBounds({
                    x: this.internalPosition.x + oldOrigin.x - newOrigin.x,
                    y: this.internalPosition.y + oldOrigin.y - newOrigin.y
                }, squares);
                return newOrigin;
            };

            this.rotation = 0;
            if (card == null) {
                updatePosition([]);
                this.activeCard = card;
            } else {
                const squares = card.squares;
                const emptyColumns = new Set();
                for (let i = 0; i < Constants.CARD_GRID_SIZE; i++) {
                    if (squares
                        .filter((square, squareIndex) => squareIndex % Constants.CARD_GRID_SIZE === i)
                        .every(square => square === CardSquareType.EMPTY)
                    ) {
                        emptyColumns.add(i);
                    }
                }

                // Removes empty rows and columns
                // todo: this should be done when we import cards into a database
                const normalizedSquares = chunk(squares, Constants.CARD_GRID_SIZE)
                    .filter(row => row.some(square => square !== CardSquareType.EMPTY))
                    .map(row => row.filter((square, index) => !emptyColumns.has(index)))
                    .reverse();

                const origin = updatePosition(normalizedSquares);

                this.activeCard = {
                    ...card,
                    origin,
                    squares: normalizedSquares
                };
            }
        },

        nextRotationStep() {
            if (this.activeCard != null) {
                this.activeCard.squares = rotateClockwise(this.activeCard.squares);
                this.rotation = this.rotation === 270 ? 0 : this.rotation + 90;
            }
        },
        previousRotationStep() {
            if (this.activeCard != null) {
                this.activeCard.squares = rotateCounterclockwise(this.activeCard.squares);
                this.rotation = this.rotation === 0 ? 270 : this.rotation - 90;
            }
        },

        setPosition(newValue: Position) {
            const origin = this.activeCard?.origin ?? { x: 0, y: 0 };
            this.internalPosition = {
                x: newValue.x - origin.x,
                y: newValue.y - origin.y
            };
        },
        applyDeltaIfPossible(positionDelta: Position) {
            const newPosition = {
                x: this.internalPosition.x + positionDelta.x,
                y: this.internalPosition.y + positionDelta.y
            };

            // Prevent any new tiles from moving out of bounds. Tiles already out of bounds moving around is ok.
            const gameBoardStore = useGameBoardStore();
            if (gameBoardStore.cardIsOutOfBounds(newPosition)) {
                const activeCardSquares = this.activeCard.squares;
                const cardSize = { width: activeCardSquares[0].length, height: activeCardSquares.length };
                const squaresUnderCurrentPosition = gameBoardStore.boardSquaresUnderCard(this.position, cardSize);
                const squaresUnderNewPosition = gameBoardStore.boardSquaresUnderCard(this.withRotationOffset(newPosition), cardSize);

                for (let y = 0; y < activeCardSquares.length; y++) {
                    for (let x = 0; x < activeCardSquares[0].length; x++) {
                        const cardSquare = activeCardSquares[y][x];
                        if (cardSquare === CardSquareType.EMPTY) continue;

                        const oldBoardSquare = squaresUnderCurrentPosition[y][x];
                        const newBoardSquare = squaresUnderNewPosition[y][x];

                        if (oldBoardSquare !== MapSquareType.OUT_OF_BOUNDS && newBoardSquare === MapSquareType.OUT_OF_BOUNDS) {
                            return;
                        }
                    }
                }
            }

            this.internalPosition = newPosition;
        },
        moveUp() {
            this.applyDeltaIfPossible({ x: 0, y: -1 });
        },
        moveDown() {
            this.applyDeltaIfPossible({ x: 0, y: 1 });
        },
        moveLeft() {
            this.applyDeltaIfPossible({ x: -1, y: 0 });
        },
        moveRight() {
            this.applyDeltaIfPossible({ x: 1, y: 0 });
        }
    }
});
