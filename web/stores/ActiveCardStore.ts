import { Card } from '~/types/Card';
import { Position } from '~/types/Position';
import { ActiveCard } from '~/types/ActiveCard';
import Constants from '~/data/Constants';
import { CardSquareType } from '~/types/CardSquareType';
import chunk from 'lodash/chunk';
import { CardRotation } from '~/types/CardRotation';
import { rotateClockwise, rotateCounterclockwise } from '~/helpers/ArrayHelper';
import { defineStore } from 'pinia';
import { useGameBoardStore } from '~/stores/GameBoardStore';
import { MapSquareType } from '~/types/MapSquareType';
import { getRotationOffset, withinBoardBounds } from '~/helpers/ActiveCardHelper';

interface ActiveCardStore {
    activeCard: ActiveCard | null
    position: Position
    rotation: CardRotation
}

export interface CardSize {
    height: number
    width: number
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
        cardSizeWithoutRotation: (state): CardSize => {
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

                const oldRotationOffset = getRotationOffset(this.rotation, this.cardSizeWithoutRotation);
                const newRotationOffset = getRotationOffset(0, { width: cardWidth, height: cardHeight });

                this.position = withinBoardBounds({
                    x: this.position.x + oldOrigin.x - newOrigin.x - oldRotationOffset.x + newRotationOffset.x,
                    y: this.position.y + oldOrigin.y - newOrigin.y - oldRotationOffset.y + newRotationOffset.y
                }, squares);
                this.rotation = 0;
                return newOrigin;
            };

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

        updateRotationValue(newValue: CardRotation) {
            const oldValue = this.rotation;
            this.rotation = newValue;
            const oldOffset = getRotationOffset(oldValue, this.cardSizeWithoutRotation);
            const newOffset = getRotationOffset(newValue, this.cardSizeWithoutRotation);

            this.position = withinBoardBounds({
                x: this.position.x - oldOffset.x + newOffset.x,
                y: this.position.y - oldOffset.y + newOffset.y
            }, this.activeCard.squares);
        },
        nextRotationStep() {
            if (this.activeCard != null) {
                this.activeCard.squares = rotateClockwise(this.activeCard.squares);
                this.updateRotationValue(this.rotation === 270 ? 0 : this.rotation + 90);
            }
        },
        previousRotationStep() {
            if (this.activeCard != null) {
                this.activeCard.squares = rotateCounterclockwise(this.activeCard.squares);
                this.updateRotationValue(this.rotation === 0 ? 270 : this.rotation - 90 as CardRotation);
            }
        },

        setPositionFromCardOrigin(newValue: Position) {
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

            // Prevent any new tiles from moving out of bounds. Tiles already out of bounds moving around is ok.
            const gameBoardStore = useGameBoardStore();
            const activeCardSquares = this.activeCard.squares;
            const cardSize = { width: activeCardSquares[0].length, height: activeCardSquares.length };
            if (gameBoardStore.cardIsOutOfBounds(newPosition, cardSize)) {
                const squaresUnderCurrentPosition = gameBoardStore.boardSquaresUnderCard(this.position, cardSize);
                const squaresUnderNewPosition = gameBoardStore.boardSquaresUnderCard(newPosition, cardSize);

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
