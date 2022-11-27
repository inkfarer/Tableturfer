import { MapSquareType } from '~/types/MapSquareType';
import { defineStore } from 'pinia';
import { GameMap } from '~/types/GameMap';
import {
    count2D,
    every2D,
    fill2D,
    findIndex2D,
    forEach2D,
    normalizeCardSquares,
    rotateClockwiseBy,
    slice2D,
    some2D
} from '~/helpers/ArrayHelper';
import { CardSize, useActiveCardStore } from '~/stores/ActiveCardStore';
import { Position } from '~/types/Position';
import { CardSquareType } from '~/types/CardSquareType';
import cloneDeep from 'lodash/cloneDeep';
import { PlayerTeam } from '~/types/PlayerTeam';
import * as Maps from '~/data/maps';
import { useRoomStore } from '~/stores/RoomStore';
import { PlayerMove } from '~/types/socket/SocketCommon';
import * as Cards from '~/data/cards';
import { Card } from '~/types/Card';
import { isFillSquare, isSpecialSquare, mapSquareFromCardSquare } from '~/helpers/SquareHelper';
import { activateSpecialSquares } from '~/helpers/BoardHelper';

interface GameBoardStore {
    name: string
    board: MapSquareType[][] | null
    usedSpecialPoints: { [key in PlayerTeam]: number }
}

export const useGameBoardStore = defineStore('gameBoard', {
    state: (): GameBoardStore => ({
        name: 'unknown',
        board: null,
        usedSpecialPoints: {
            [PlayerTeam.ALPHA]: 0,
            [PlayerTeam.BRAVO]: 0
        }
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
            return (position: Position, cardSquares: CardSquareType[][] | null, team: PlayerTeam | null) => {
                if (cardSquares == null || this.board == null || team == null) {
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

                const acceptedNearbyBoardSquares = team === PlayerTeam.ALPHA
                    ? [MapSquareType.FILL_ALPHA, MapSquareType.INACTIVE_SPECIAL_ALPHA, MapSquareType.ACTIVE_SPECIAL_ALPHA]
                    : [MapSquareType.FILL_BRAVO, MapSquareType.INACTIVE_SPECIAL_BRAVO, MapSquareType.ACTIVE_SPECIAL_BRAVO];

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
                        this.board as MapSquareType[][],
                        { x: position.x - 1 + x, y: position.y - 1 + y },
                        { x: position.x + 1 + x, y: position.y + 1 + y });

                    return some2D(boardSquaresAroundCardSquare, square => acceptedNearbyBoardSquares.includes(square));
                });
            };
        },
        boardSquaresUnderCard() {
            return (position: Position, cardSize: CardSize) => {
                return slice2D(
                    this.board ?? [],
                    position,
                    { x: position.x + cardSize.width - 1, y: position.y + cardSize.height - 1 },
                    MapSquareType.OUT_OF_BOUNDS);
            };
        },
        cardIsOutOfBounds() {
            return (position: Position, cardSize: CardSize) => {
                return position.x < 0 || position.y < 0
                    || position.x + cardSize.width > this.boardSize.width
                    || position.y + cardSize.height > this.boardSize.height;
            };
        },
        specialPointCount(): { [key in PlayerTeam]: number } {
            const result = { [PlayerTeam.ALPHA]: 0, [PlayerTeam.BRAVO]: 0 };

            if (this.board == null) {
                return result;
            }

            forEach2D(this.board, square => {
                if (square === MapSquareType.ACTIVE_SPECIAL_ALPHA) {
                    result[PlayerTeam.ALPHA]++;
                } else if (square === MapSquareType.ACTIVE_SPECIAL_BRAVO) {
                    result[PlayerTeam.BRAVO]++;
                }
            });

            return result;
        }
    },
    actions: {
        setBoardByName(boardName: string) {
            const map = (Maps as Record<string, GameMap>)[boardName];
            if (map != null) {
                this.setBoard(map);
            } else {
                throw new Error(`Unknown map "${boardName}"`);
            }
        },
        setBoard(map: GameMap) {
            this.name = map.name;
            this.board = map.squares;
            this.usedSpecialPoints = { [PlayerTeam.ALPHA]: 0, [PlayerTeam.BRAVO]: 0 };

            const playerTeam = useRoomStore().playerTeam;
            if (playerTeam != null) {
                const startSquarePosition = findIndex2D(map.squares, square =>
                    square === (playerTeam === PlayerTeam.ALPHA
                        ? MapSquareType.INACTIVE_SPECIAL_ALPHA
                        : MapSquareType.INACTIVE_SPECIAL_BRAVO));
                if (startSquarePosition != null) {
                    const activeCardStore = useActiveCardStore();
                    activeCardStore.setPositionFromCardOrigin(startSquarePosition);
                }
            }
        },
        applyMoves(moves: { [team in PlayerTeam]: PlayerMove }) {
            const boardUpdates = fill2D(this.boardSize.width, this.boardSize.height, MapSquareType.EMPTY);

            const movesWithCards = Object.entries(moves).map(([team, move]) => {
                const card = (Cards as Record<string, Card>)[move.cardName];
                if (card == null) {
                    throw new Error(`Unknown card "${move.cardName}"`);
                }
                const normalizedSquares = rotateClockwiseBy(normalizeCardSquares(card.squares), move.rotation);

                return {
                    ...move,
                    team: team as PlayerTeam,
                    card: {
                        ...card,
                        squares: normalizedSquares
                    },
                    cardSquareCount: count2D(normalizedSquares, square => square !== CardSquareType.EMPTY)
                };
            });
            movesWithCards.sort((a, b) => b.cardSquareCount - a.cardSquareCount);

            const squareCountsMatch = movesWithCards.every(move => move.cardSquareCount === movesWithCards[0].cardSquareCount);

            movesWithCards.forEach(move => {
                if (move.special) {
                    this.usedSpecialPoints[move.team] += move.card.specialCost;
                }

                forEach2D(move.card.squares, (square, position) => {
                    if (square === CardSquareType.EMPTY) {
                        return;
                    }

                    const boardPosition = { x: move.position.x + position.x, y: move.position.y + position.y };
                    const existingSquare = boardUpdates[boardPosition.y][boardPosition.x];
                    let newSquare = mapSquareFromCardSquare(square, move.team);

                    if (isSpecialSquare(existingSquare) && isFillSquare(newSquare)) {
                        return;
                    }

                    if (squareCountsMatch
                        && ((isFillSquare(existingSquare) && isFillSquare(newSquare))
                            || (isSpecialSquare(existingSquare) && isSpecialSquare(newSquare))))
                    {
                        newSquare = MapSquareType.NEUTRAL;
                    }

                    boardUpdates[boardPosition.y][boardPosition.x] = newSquare;
                });
            });

            const newBoard = cloneDeep(this.board) as MapSquareType[][];
            forEach2D(boardUpdates, (square, position) => {
                if (square === MapSquareType.EMPTY) {
                    return;
                }

                newBoard[position.y][position.x] = square;
            });
            activateSpecialSquares(newBoard);
            this.board = newBoard;
        }
    }
});
