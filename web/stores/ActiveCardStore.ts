import { Card } from '~/types/Card';
import { Position } from '~/types/Position';
import { ActiveCard } from '~/types/ActiveCard';
import Constants from '~/data/Constants';
import { CardSquareType } from '~/types/CardSquareType';
import chunk from 'lodash/chunk';
import { CardRotation } from '~/types/CardRotation';
import { every2D, rotateClockwise, rotateCounterclockwise } from '~/helpers/ArrayHelper';
import { defineStore } from 'pinia';
import { useGameBoardStore } from '~/stores/GameBoardStore';
import { MapSquareType } from '~/types/MapSquareType';

interface ActiveCardStore {
    activeCard: ActiveCard | null
    position: Position
    rotation: CardRotation
}

export interface CardSize {
    height: number
    width: number
}

function getCardOrigin(width: number, height: number): Position {
    return {
        x: Math.ceil(width / 2 - 1),
        y: Math.floor(height / 2)
    };
}

export const useActiveCardStore = defineStore('activeCard', {
    state: (): ActiveCardStore => ({
        activeCard: null,
        position: {
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
        // Strictly matches the way the card moves as it is rotated in-game. Can probably be cleaned up.
        offsetPosition(): Position {
            const rotation = this.rotation;
            if (rotation === 0) {
                return this.position;
            } else {
                const { width, height } = this.cardSize;
                if (width === height) {
                    return this.position;
                }

                const addToPosition = (x: number, y: number): Position => ({
                    x: this.position.x + x,
                    y: this.position.y + y
                });

                switch (rotation) {
                    case 90: {
                        let x = Math.ceil((width - height) / 2);
                        const y = Math.ceil((height - width) / 2);

                        if (height % 2 === 1 && width % 2 === 0) {
                            x -= 1;
                        }

                        return addToPosition(x, y);
                    }
                    case 180: {
                        if (height % 2 === 0 && width % 2 === 1) {
                            return addToPosition(0, (height + width) % 2);
                        } else {
                            return addToPosition(((height + width) % 2) * -1, 0);
                        }
                    }
                    case 270: {
                        const x = Math.floor((width - height) / 2);
                        let y = x * -1;

                        if (height % 2 === 1 && width % 2 === 0) {
                            y -= 1;
                        }

                        return addToPosition(x, y);
                    }
                }
            }
        }
    },
    actions: {
        setActiveCard(card: Card | null) {
            const setOrigin = (newOrigin = { x: 0, y: 0 }) => {
                const oldOrigin = this.activeCard?.origin ?? { x: 0, y: 0 };
                // todo: when switching cards, the game does not let a card be entirely out of bounds
                this.position = {
                    x: this.position.x + oldOrigin.x - newOrigin.x,
                    y: this.position.y + oldOrigin.y - newOrigin.y
                };
            };

            this.rotation = 0;
            if (card == null) {
                setOrigin();
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

                const origin = getCardOrigin(normalizedSquares[0].length, normalizedSquares.length);
                setOrigin(origin);

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
            this.position = {
                x: newValue.x - origin.x,
                y: newValue.y - origin.y
            };
        },
        applyDeltaIfPossible(positionDelta: Position) {
            const newPosition = {
                x: this.position.x + positionDelta.x,
                y: this.position.y + positionDelta.y
            };

            const gameBoardStore = useGameBoardStore();
            if (gameBoardStore.cardIsOutOfBounds(newPosition)) {
                const squaresUnderCurrentPosition = gameBoardStore.boardSquaresUnderCard(this.position);
                const squaresUnderNewPosition = gameBoardStore.boardSquaresUnderCard(newPosition);

                for (let y = 0; y < this.activeCard.squares.length; y++) {
                    for (let x = 0; x < this.activeCard.squares[0].length; x++) {
                        const cardSquare = this.activeCard.squares[y][x];
                        if (cardSquare === CardSquareType.EMPTY) continue;

                        const oldBoardSquare = squaresUnderCurrentPosition[y][x];
                        const newBoardSquare = squaresUnderNewPosition[y][x];

                        if (oldBoardSquare !== MapSquareType.OUT_OF_BOUNDS && newBoardSquare === MapSquareType.OUT_OF_BOUNDS) {
                            return;
                        }
                    }
                }
            }

            this.position = newPosition;
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
