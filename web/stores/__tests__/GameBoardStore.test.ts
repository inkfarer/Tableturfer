import { setActivePinia } from 'pinia';
import { MapSquareType, MapSquareType as MST } from '~/types/MapSquareType';
import { CardSquareType as CST } from '~/types/CardSquareType';
import { useGameBoardStore } from '~/stores/GameBoardStore';
import { useActiveCardStore } from '~/stores/ActiveCardStore';
import { createTestingPinia } from '@pinia/testing';
import { PlayerTeam } from '~/types/PlayerTeam';
import { useRoomStore } from '~/stores/RoomStore';
import { fill2D } from '~/helpers/ArrayHelper';

describe('GameBoardStore', () => {
    beforeEach(() => {
        setActivePinia(createTestingPinia({
            stubActions: false
        }));
    });

    describe('getters', () => {
        describe('boardSize', () => {
            it('returns 0 as the width and height if the board is unset', () => {
                const store = useGameBoardStore();
                store.board = null;

                expect(store.boardSize).toEqual({ width: 0, height: 0 });
            });

            it('returns the width and height of the board', () => {
                const store = useGameBoardStore();
                store.board = [
                    [MST.EMPTY, MST.FILL_BRAVO],
                    [MST.FILL_ALPHA, MST.FILL_BRAVO],
                    [MST.SPECIAL_BRAVO, MST.FILL_BRAVO]
                ];

                expect(store.boardSize).toEqual({ width: 2, height: 3 });
            });
        });

        describe('isPlaceable', () => {
            const board = [
                [MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED],
                [MST.DISABLED, MST.SPECIAL_ALPHA, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.FILL_ALPHA, MST.DISABLED],
                [MST.DISABLED, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.DISABLED],
                [MST.DISABLED, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.DISABLED],
                [MST.DISABLED, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.DISABLED],
                [MST.DISABLED, MST.SPECIAL_BRAVO, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.FILL_BRAVO, MST.DISABLED],
                [MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED]
            ];
            const card = [
                [CST.EMPTY, CST.FILL],
                [CST.EMPTY, CST.SPECIAL],
                [CST.FILL, CST.FILL]
            ];
            const card2 = [
                [CST.FILL, CST.FILL],
                [CST.EMPTY, CST.SPECIAL],
                [CST.EMPTY, CST.FILL]
            ];
            const card3 = [
                [CST.FILL]
            ];

            beforeEach(() => {
                useGameBoardStore().board = board;
            });

            it('returns false if the board has not been initialized', () => {
                const store = useGameBoardStore();
                store.board = null;

                expect(store.isPlaceable({ x: 2, y: 1 }, card, PlayerTeam.ALPHA)).toBe(false);
            });

            it('returns false if no card is passed in', () => {
                expect(useGameBoardStore().isPlaceable({ x: 2, y: 1 }, null, PlayerTeam.ALPHA)).toBe(false);
            });

            it('returns false if the player has no team set', () => {
                expect(useGameBoardStore().isPlaceable({ x: 2, y: 1 }, card3, null)).toBe(false);
            });

            describe.each([PlayerTeam.ALPHA, PlayerTeam.BRAVO])('common for both teams [%s]', team => {
                it.each([
                    { x: 2, y: -3 },
                    { x: 2, y: 15 },
                    { x: -2, y: 2 },
                    { x: 12, y: 2 }
                ])('returns false if the card is out of bounds ($x, $y)', ({ x, y }) => {
                    expect(useGameBoardStore().isPlaceable({ x, y }, card, team)).toBe(false);
                });

                it.each([
                    { x: 1, y: 0 },
                    { x: 0, y: 2 },
                    { x: 5, y: 2 },
                    { x: 2, y: 4 }
                ])('returns false if the card is on top of disabled tiles ($x, $y)', ({ x, y }) => {
                    expect(useGameBoardStore().isPlaceable({ x, y }, card, team)).toBe(false);
                });

                it.each([
                    { x: 1, y: 3 },
                    { x: 5, y: 3 },
                    { x: 3, y: 1 },
                    { x: 3, y: 5 },
                    { x: 3, y: 3 }
                ])('returns false if the card is not adjacent to any tiles, only on top of empty tiles ($x, $y)', ({ x, y }) => {
                    expect(useGameBoardStore().isPlaceable({ x, y }, card3, team)).toBe(false);
                });

                it.each([
                    { x: 1, y: 1, card: card2 },
                    { x: 4, y: 1, card },
                    { x: 1, y: 3, card },
                    { x: 4, y: 3, card }
                ])('returns false if the card is on top of fill or special tiles ($x, $y)', ({ x, y, card }) => {
                    expect(useGameBoardStore().isPlaceable({ x, y }, card, team)).toBe(false);
                });
            });

            describe('alpha team', () => {
                it.each([
                    { x: 2, y: 3 },
                    { x: 3, y: 3 }
                ])('returns false if the card is next to the opposing side\'s tiles ($x, $y)', ({ x, y }) => {
                    expect(useGameBoardStore().isPlaceable({ x, y }, card, PlayerTeam.ALPHA)).toBe(false);
                });

                it.each([
                    { x: 1, y: 1 },
                    { x: 3, y: 1 }
                ])('returns true if the card is next to the player\'s tiles ($x, $y)', ({ x, y }) => {
                    expect(useGameBoardStore().isPlaceable({ x, y }, card, PlayerTeam.ALPHA)).toBe(true);
                });
            });

            describe('bravo team', () => {
                beforeEach(() => {
                    // @ts-ignore
                    useRoomStore().playerTeam = PlayerTeam.BRAVO;
                });

                it.each([
                    { x: 1, y: 1 },
                    { x: 3, y: 1 }
                ])('returns false if the card is next to the opposing side\'s tiles ($x, $y)', ({ x, y }) => {
                    expect(useGameBoardStore().isPlaceable({ x, y }, card, PlayerTeam.BRAVO)).toBe(false);
                });

                it.each([
                    { x: 2, y: 3 },
                    { x: 3, y: 3 }
                ])('returns true if the card is next to the player\'s tiles ($x, $y)', ({ x, y }) => {
                    expect(useGameBoardStore().isPlaceable({ x, y }, card, PlayerTeam.BRAVO)).toBe(true);
                });
            });
        });

        describe('boardSquaresUnderCard', () => {
            it('returns board squares under the given position for a card of the given size', () => {
                const gameBoardStore = useGameBoardStore();
                gameBoardStore.board = [
                    [MST.FILL_ALPHA, MST.FILL_BRAVO],
                    [MST.EMPTY, MST.FILL_BRAVO],
                    [MST.FILL_ALPHA, MST.EMPTY],
                    [MST.FILL_BRAVO, MST.FILL_ALPHA]
                ];

                const result = gameBoardStore.boardSquaresUnderCard({ x: 0, y: 3 }, { width: 3, height: 2 });

                expect(result).toEqual([
                    [MST.FILL_BRAVO, MST.FILL_ALPHA, MST.OUT_OF_BOUNDS],
                    [MST.OUT_OF_BOUNDS, MST.OUT_OF_BOUNDS, MST.OUT_OF_BOUNDS]
                ]);
            });
        });

        describe('cardIsOutOfBounds', () => {
            const cardSize = { height: 3, width: 4 };

            beforeEach(() => {
                // @ts-ignore
                useGameBoardStore().boardSize = { width: 10, height: 5 };
            });

            it.each([
                [true, -1, 0],
                [false, 0, 0],
                [true, 0, -1],
                [false, 6, 0],
                [true, 7, 0],
                [false, 0, 2],
                [true, 0, 3],
                [false, 6, 2],
                [true, 6, 3]
            ])('is %s if the position is (%d, %d)', (expectedResult, x, y) => {
                expect(useGameBoardStore().cardIsOutOfBounds({ x, y }, cardSize)).toEqual(expectedResult);
            });
        });
    });

    describe('actions', () => {
        describe('setBoard', () => {
            it('updates the board', () => {
                const store = useGameBoardStore();
                const squares = [
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY]
                ];

                store.setBoard({
                    name: 'cool-map',
                    squares
                });

                expect(store.name).toEqual('cool-map');
                expect(store.board).toEqual(squares);
            });

            it.each([
                [PlayerTeam.ALPHA, 2, 1],
                [PlayerTeam.BRAVO, 3, 2]
            ])('updates the position of the active card to the location of the starting square for team %s', (team, expectedX, expectedY) => {
                // @ts-ignore
                useRoomStore().playerTeam = team;
                const activeCardStore = useActiveCardStore();
                jest.spyOn(activeCardStore, 'setPositionFromCardOrigin');
                const gameBoardStore = useGameBoardStore();
                const squares = [
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.SPECIAL_ALPHA, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.SPECIAL_BRAVO, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY]
                ];

                gameBoardStore.setBoard({
                    name: 'cool-map',
                    squares
                });

                expect(activeCardStore.setPositionFromCardOrigin).toHaveBeenCalledWith({ x: expectedX, y: expectedY });
            });
        });

        describe('applyMoves', () => {
            beforeEach(() => {
                useGameBoardStore().board = fill2D(6, 6, MapSquareType.EMPTY);
            });

            it('applies the chosen moves to the board', () => {
                const store = useGameBoardStore();

                store.applyMoves({
                    [PlayerTeam.ALPHA]: {
                        cardName: 'BombCurling',
                        position: { x: 1, y: 1 },
                        rotation: 270
                    },
                    [PlayerTeam.BRAVO]: {
                        cardName: 'SaberLight00',
                        position: { x: 3, y: 1 },
                        rotation: 0
                    }
                });

                expect(store.board).toEqual([
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.FILL_ALPHA, MST.SPECIAL_BRAVO, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.SPECIAL_ALPHA, MST.FILL_ALPHA, MST.FILL_BRAVO, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.FILL_ALPHA, MST.FILL_BRAVO, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.FILL_BRAVO, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.FILL_BRAVO, MST.EMPTY, MST.EMPTY]
                ]);
            });

            it('behaves as expected with two identical moves', () => {
                const store = useGameBoardStore();

                store.applyMoves({
                    [PlayerTeam.ALPHA]: {
                        cardName: 'BombCurling',
                        position: { x: 1, y: 1 },
                        rotation: 270
                    },
                    [PlayerTeam.BRAVO]: {
                        cardName: 'BombCurling',
                        position: { x: 1, y: 1 },
                        rotation: 270
                    }
                });

                expect(store.board).toEqual([
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.NEUTRAL, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.NEUTRAL, MST.NEUTRAL, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.NEUTRAL, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY]
                ]);
            });

            it('behaves as expected when overlapping moves have the same cost', () => {
                const store = useGameBoardStore();

                store.applyMoves({
                    [PlayerTeam.ALPHA]: {
                        cardName: 'BombCurling',
                        position: { x: 2, y: 2 },
                        rotation: 90
                    },
                    [PlayerTeam.BRAVO]: {
                        cardName: 'BombCurling',
                        position: { x: 1, y: 2 },
                        rotation: 180
                    }
                });

                expect(store.board).toEqual([
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.FILL_BRAVO, MST.NEUTRAL, MST.FILL_BRAVO, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.SPECIAL_BRAVO, MST.SPECIAL_ALPHA, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.FILL_ALPHA, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY]
                ]);
            });

            it('behaves as expected when applying overlapping moves over existing squares', () => {
                const store = useGameBoardStore();
                store.board![0][0] = MapSquareType.FILL_BRAVO;
                store.board![1][2] = MapSquareType.FILL_BRAVO;
                store.board![2][3] = MapSquareType.FILL_ALPHA;
                store.board![2][2] = MapSquareType.FILL_ALPHA;

                store.applyMoves({
                    [PlayerTeam.ALPHA]: {
                        cardName: 'BombCurling',
                        position: { x: 2, y: 2 },
                        rotation: 90
                    },
                    [PlayerTeam.BRAVO]: {
                        cardName: 'BombCurling',
                        position: { x: 1, y: 2 },
                        rotation: 180
                    }
                });

                expect(store.board).toEqual([
                    [MST.FILL_BRAVO, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.FILL_BRAVO, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.FILL_BRAVO, MST.NEUTRAL, MST.FILL_BRAVO, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.SPECIAL_BRAVO, MST.SPECIAL_ALPHA, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.FILL_ALPHA, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY]
                ]);
            });

            it('behaves as expected when overlapping moves have differing costs', () => {
                const store = useGameBoardStore();

                store.applyMoves({
                    [PlayerTeam.ALPHA]: {
                        cardName: 'BombCurling',
                        position: { x: 1, y: 1 },
                        rotation: 90
                    },
                    [PlayerTeam.BRAVO]: {
                        cardName: 'SaberLight00',
                        position: { x: 1, y: 1 },
                        rotation: 0
                    }
                });

                expect(store.board).toEqual([
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.SPECIAL_BRAVO, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.FILL_ALPHA, MST.SPECIAL_ALPHA, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.FILL_ALPHA, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.FILL_BRAVO, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.FILL_BRAVO, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY]
                ]);
            });

            it('behaves as expected when overlapping moves have differing costs, regardless of the order turns appear in', () => {
                const store = useGameBoardStore();

                store.applyMoves({
                    [PlayerTeam.BRAVO]: {
                        cardName: 'SaberLight00',
                        position: { x: 1, y: 1 },
                        rotation: 0
                    },
                    [PlayerTeam.ALPHA]: {
                        cardName: 'BombCurling',
                        position: { x: 1, y: 1 },
                        rotation: 90
                    }
                });

                expect(store.board).toEqual([
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.SPECIAL_BRAVO, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.FILL_ALPHA, MST.SPECIAL_ALPHA, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.FILL_ALPHA, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.FILL_BRAVO, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.FILL_BRAVO, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY]
                ]);
            });

            it('behaves as expected when special squares overlap in moves with different costs', () => {
                const store = useGameBoardStore();

                store.applyMoves({
                    [PlayerTeam.ALPHA]: {
                        cardName: 'BombCurling',
                        position: { x: 0, y: 0 },
                        rotation: 90
                    },
                    [PlayerTeam.BRAVO]: {
                        cardName: 'SaberLight00',
                        position: { x: 1, y: 1 },
                        rotation: 0
                    }
                });

                expect(store.board).toEqual([
                    [MST.FILL_ALPHA, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.FILL_ALPHA, MST.SPECIAL_ALPHA, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.FILL_ALPHA, MST.FILL_BRAVO, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.FILL_BRAVO, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.FILL_BRAVO, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.FILL_BRAVO, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY]
                ]);
            });

            it('behaves as expected when special squares overlap in moves with different costs, regardless of the order turns appear in', () => {
                const store = useGameBoardStore();

                store.applyMoves({
                    [PlayerTeam.BRAVO]: {
                        cardName: 'SaberLight00',
                        position: { x: 1, y: 1 },
                        rotation: 0
                    },
                    [PlayerTeam.ALPHA]: {
                        cardName: 'BombCurling',
                        position: { x: 0, y: 0 },
                        rotation: 90
                    }
                });

                expect(store.board).toEqual([
                    [MST.FILL_ALPHA, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.FILL_ALPHA, MST.SPECIAL_ALPHA, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.FILL_ALPHA, MST.FILL_BRAVO, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.FILL_BRAVO, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.FILL_BRAVO, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.FILL_BRAVO, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY]
                ]);
            });
        });
    });
});
