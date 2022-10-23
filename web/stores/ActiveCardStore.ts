import { Card } from '~/types/Card';
import { Position } from '~/types/Position';
import { ActiveCard } from '~/types/ActiveCard';
import Constants from '~/data/Constants';
import { CardSquareType } from '~/types/CardSquareType';
import chunk from 'lodash/chunk';
import { CardRotation } from '~/types/CardRotation';
import { rotateClockwise, rotateCounterclockwise } from '~/helpers/ArrayHelper';
import { defineStore } from 'pinia';

interface ActiveCardStore {
    activeCard: ActiveCard | null
    position: Position
    rotation: CardRotation
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
        cardSize: state => {
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
            const setOrigin = (newOrigin = { x: 0, y: 0 }) => {
                const oldOrigin = this.activeCard?.origin ?? { x: 0, y: 0 };
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

                const width = normalizedSquares[0].length;
                const height = normalizedSquares.length;
                const origin = {
                    x: Math.ceil(width / 2 - 1),
                    y: Math.floor(height / 2)
                };
                setOrigin(origin);

                this.activeCard = {
                    ...card,
                    origin,
                    squares: normalizedSquares
                };
            }
        },

        nextRotationStep() {
            this.activeCard.squares = rotateClockwise(this.activeCard.squares);
            this.rotation = this.rotation === 270 ? 0 : this.rotation + 90;
        },
        previousRotationStep() {
            this.activeCard.squares = rotateCounterclockwise(this.activeCard.squares);
            this.rotation = this.rotation === 0 ? 270 : this.rotation - 90;
        },

        setPosition(newValue: Position) {
            const origin = this.activeCard?.origin ?? { x: 0, y: 0 };
            this.position = {
                x: newValue.x - origin.x,
                y: newValue.y - origin.y
            };
        },
        moveUp() {
            this.position.y--;
        },
        moveDown() {
            this.position.y++;
        },
        moveLeft() {
            this.position.x--;
        },
        moveRight() {
            this.position.x++;
        }
    }
});
