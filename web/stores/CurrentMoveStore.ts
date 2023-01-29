import { Card } from '~/types/Card';
import { Position } from '~/types/Position';
import { ActiveCard } from '~/types/ActiveCard';
import { CardSquareType } from '~/types/CardSquareType';
import { CardRotation } from '~/types/CardRotation';
import { getSize, rotateClockwise, rotateClockwiseBy, rotateCounterclockwise } from '~/helpers/ArrayHelper';
import { defineStore } from 'pinia';
import { useGameBoardStore } from '~/stores/GameBoardStore';
import { MapSquareType } from '~/types/MapSquareType';
import { getRotationOffset, withinBoardBounds } from '~/helpers/ActiveCardHelper';
import cloneDeep from 'lodash/cloneDeep';
import { useNuxtApp } from '#imports';
import { useRoomStore } from '~/stores/RoomStore';
import { useUserSettingsStore } from '~/stores/UserSettingsStore';

interface CurrentMoveStore {
    activeCard: ActiveCard | null
    position: Position
    rotation: CardRotation
    special: boolean
    pass: boolean
    locked: boolean
}

export interface CardSize {
    height: number
    width: number
}

export const useCurrentMoveStore = defineStore('currentMove', {
    state: (): CurrentMoveStore => ({
        activeCard: null,
        position: {
            x: 0,
            y: 0
        },
        rotation: 0,
        special: false,
        pass: false,
        locked: false
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
        },
        positionIsValid: (state) => (position: Position): boolean => {
            if (state.activeCard == null) {
                return false;
            }

            // Prevent any new tiles from moving out of bounds. Tiles already out of bounds moving around is ok.
            const gameBoardStore = useGameBoardStore();
            const activeCardSquares = state.activeCard.squares;
            const cardSize = { width: activeCardSquares[0].length, height: activeCardSquares.length };
            if (gameBoardStore.cardIsOutOfBounds(position, cardSize)) {
                const squaresUnderCurrentPosition = gameBoardStore.boardSquaresUnderCard(state.position, cardSize);
                const squaresUnderNewPosition = gameBoardStore.boardSquaresUnderCard(position, cardSize);

                for (let y = 0; y < activeCardSquares.length; y++) {
                    for (let x = 0; x < activeCardSquares[0].length; x++) {
                        const cardSquare = activeCardSquares[y][x];
                        if (cardSquare === CardSquareType.EMPTY) continue;

                        const oldBoardSquare = squaresUnderCurrentPosition[y][x];
                        const newBoardSquare = squaresUnderNewPosition[y][x];

                        if (oldBoardSquare !== MapSquareType.OUT_OF_BOUNDS && newBoardSquare === MapSquareType.OUT_OF_BOUNDS) {
                            return false;
                        }
                    }
                }
            }

            return true;
        }
    },
    actions: {
        setActiveCard(card: Card | null) {
            if (this.locked) {
                return;
            }

            if (card == null) {
                this.activeCard = null;
            } else {
                const cardSquares = cloneDeep(card.squares);

                const oldOrigin = this.activeCard?.origin ?? { x: 0, y: 0 };

                const cardWidth = cardSquares[0]?.length ?? 0;
                const cardHeight = cardSquares.length;
                const newOrigin = {
                    x: Math.ceil(cardWidth / 2 - 1),
                    y: Math.floor(cardHeight / 2)
                };

                const oldRotationOffset = getRotationOffset(this.rotation, this.cardSizeWithoutRotation);
                const newRotationOffset = getRotationOffset(0, { width: cardWidth, height: cardHeight });

                this.position = withinBoardBounds({
                    x: this.position.x + oldOrigin.x - newOrigin.x - oldRotationOffset.x + newRotationOffset.x,
                    y: this.position.y + oldOrigin.y - newOrigin.y - oldRotationOffset.y + newRotationOffset.y
                }, cardSquares);
                this.rotation = 0;

                this.activeCard = {
                    ...card,
                    origin: newOrigin,
                    squares: cardSquares
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
            }, this.activeCard?.squares);
        },
        nextRotationStep() {
            if (this.activeCard != null) {
                this.activeCard.squares = rotateClockwise(this.activeCard.squares);
                this.updateRotationValue(this.rotation === 270 ? 0 : this.rotation + 90 as CardRotation);
            }
        },
        previousRotationStep() {
            if (this.activeCard != null) {
                this.activeCard.squares = rotateCounterclockwise(this.activeCard.squares);
                this.updateRotationValue(this.rotation === 0 ? 270 : this.rotation - 90 as CardRotation);
            }
        },

        getPositionFromCardOrigin(position: Position) {
            const origin = this.activeCard?.origin ?? { x: 0, y: 0 };
            return {
                x: position.x - origin.x,
                y: position.y - origin.y
            };
        },
        setPositionFromCardOrigin(newValue: Position) {
            this.position = this.getPositionFromCardOrigin(newValue);
        },
        normalizePositionIfMovementAllowed(position: Position, fromOrigin: boolean): Position | null {
            if (this.activeCard == null || this.locked) {
                return null;
            }

            if (isNaN(position.x) || isNaN(position.y)) {
                console.warn('Ignoring attempt to move position by NaN tiles');
                return null;
            }

            const normalizedPosition = fromOrigin ? this.getPositionFromCardOrigin(position) : position;
            if (this.position.x === normalizedPosition.x && this.position.y === normalizedPosition.y) {
                return null;
            }

            return normalizedPosition;
        },
        setPositionInsideBoard(newPosition: Position, fromOrigin: boolean) {
            const normalizedPosition = this.normalizePositionIfMovementAllowed(newPosition, fromOrigin);
            if (normalizedPosition == null) {
                return;
            }

            // const updatedPosition = cloneDeep(this.position);
            // if (this.positionIsValid({ x: updatedPosition.x, y: normalizedPosition.y })) {
            //     updatedPosition.y = normalizedPosition.y;
            // }
            // if (this.positionIsValid({ x: normalizedPosition.x, y: updatedPosition.y })) {
            //     updatedPosition.x = normalizedPosition.x;
            // }

            const gameBoardStore = useGameBoardStore();
            const cardSize = getSize(this.activeCard?.squares ?? []);
            const boardSize = gameBoardStore.boardSize;

            this.position = {
                x: Math.max(Math.min(normalizedPosition.x, boardSize.width - cardSize.width), Math.min(this.position.x, 0)),
                y: Math.max(Math.min(normalizedPosition.y, boardSize.height - cardSize.height), Math.min(this.position.y, 0))
            };
        },
        setPositionIfPossible(newPosition: Position, fromOrigin: boolean) {
            const normalizedPosition = this.normalizePositionIfMovementAllowed(newPosition, fromOrigin);
            if (normalizedPosition == null) {
                return;
            }

            if (this.positionIsValid(normalizedPosition)) {
                this.position = normalizedPosition;
            }
        },
        applyDeltaIfPossible(positionDelta: Position) {
            if (positionDelta.x === 0 && positionDelta.y === 0) {
                return;
            }

            const newPosition = {
                x: this.position.x + positionDelta.x,
                y: this.position.y + positionDelta.y
            };

            this.setPositionIfPossible(newPosition, false);
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
        },
        resetGame() {
            this.$reset();
        },
        setSpecial(special: boolean) {
            if (!this.locked) {
                if (special) {
                    this.pass = false;
                }
                this.special = special;
            }
        },
        setPass(pass: boolean) {
            if (!this.locked) {
                if (pass) {
                    this.special = false;
                }
                this.pass = pass;
            }
        },
        onNewMove() {
            this.locked = false;
            this.pass = false;
            this.special = false;
            this.setActiveCard(null);
        },
        proposeMove() {
            const roomStore = useRoomStore();
            if (this.activeCard == null || roomStore.playerTeam == null || this.pass) {
                return;
            }

            const { $socket } = useNuxtApp();
            const userSettingsStore = useUserSettingsStore();
            const boardFlipped = userSettingsStore.boardFlipped;
            const rotation = boardFlipped ? (this.rotation + 180) % 360 : this.rotation;
            const position = boardFlipped ? this.getFlippedPosition(this.position) : this.position;

            $socket.send('ProposeMove', {
                type: 'PlaceCard',
                cardName: this.activeCard.name,
                position: position,
                rotation: rotation,
                special: this.special
            });
            this.locked = true;
        },

        getFlippedPosition(position: Position): Position {
            const boardSize = useGameBoardStore().boardSize;
            const cardSize = getSize(this.activeCard?.squares ?? []);

            return {
                x: boardSize.width - position.x - cardSize.width,
                y: boardSize.height - position.y - cardSize.height
            };
        },
        getFlippedRotation(rotation: CardRotation): CardRotation {
            return (rotation + 180) % 360 as CardRotation;
        },

        flipPosition() {
            if (this.activeCard) {
                this.activeCard.squares = rotateClockwiseBy(this.activeCard.squares, 180);
            }
            this.rotation = this.getFlippedRotation(this.rotation);
            this.position = this.getFlippedPosition(this.position);
        }
    }
});
