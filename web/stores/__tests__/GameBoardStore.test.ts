import { setActivePinia } from 'pinia';
import { MapSquareType, MapSquareType as MST } from '~/types/MapSquareType';
import { CardSquareType as CST } from '~/types/CardSquareType';
import { useGameBoardStore } from '~/stores/GameBoardStore';
import { useCurrentMoveStore } from '~/stores/CurrentMoveStore';
import { createTestingPinia } from '@pinia/testing';
import { PlayerTeam } from '~/types/PlayerTeam';
import { useRoomStore } from '~/stores/RoomStore';
import { fill2D } from '~/helpers/ArrayHelper';
import { activateSpecialSquares } from '~/helpers/BoardHelper';
import cloneDeep from 'lodash/cloneDeep';
import { useUserSettingsStore } from '~/stores/UserSettingsStore';

jest.mock('~/helpers/BoardHelper');

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
                    [MST.INACTIVE_SPECIAL_BRAVO, MST.FILL_BRAVO]
                ];

                expect(store.boardSize).toEqual({ width: 2, height: 3 });
            });
        });

        describe('isPlaceable', () => {
            const board = [
                [MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED],
                [MST.DISABLED, MST.INACTIVE_SPECIAL_ALPHA, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.FILL_ALPHA, MST.DISABLED],
                [MST.DISABLED, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.DISABLED],
                [MST.DISABLED, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.DISABLED],
                [MST.DISABLED, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.DISABLED],
                [MST.DISABLED, MST.INACTIVE_SPECIAL_BRAVO, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.FILL_BRAVO, MST.DISABLED],
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
                useGameBoardStore().board = cloneDeep(board);
                useCurrentMoveStore().special = false;
            });

            it('returns false if the board has not been initialized', () => {
                // @ts-ignore
                useRoomStore().playerTeam = PlayerTeam.ALPHA;
                const store = useGameBoardStore();
                store.board = null;

                expect(store.isPlaceable({ x: 2, y: 1 }, card)).toBe(false);
            });

            it('returns false if no card is passed in', () => {
                // @ts-ignore
                useRoomStore().playerTeam = PlayerTeam.ALPHA;

                expect(useGameBoardStore().isPlaceable({ x: 2, y: 1 }, null)).toBe(false);
            });

            it('returns false if the player has no team set', () => {
                // @ts-ignore
                useRoomStore().playerTeam = null;

                expect(useGameBoardStore().isPlaceable({ x: 2, y: 1 }, card3)).toBe(false);
            });

            describe.each([PlayerTeam.ALPHA, PlayerTeam.BRAVO])('common for both teams [%s]', team => {
                beforeEach(() => {
                    // @ts-ignore
                    useRoomStore().playerTeam = team;
                });

                it.each([
                    { x: 2, y: -3 },
                    { x: 2, y: 15 },
                    { x: -2, y: 2 },
                    { x: 12, y: 2 }
                ])('returns false if the card is out of bounds ($x, $y)', ({ x, y }) => {
                    expect(useGameBoardStore().isPlaceable({ x, y }, card)).toBe(false);
                });

                it.each([
                    { x: 1, y: 0 },
                    { x: 0, y: 2 },
                    { x: 5, y: 2 },
                    { x: 2, y: 4 }
                ])('returns false if the card is on top of disabled tiles ($x, $y)', ({ x, y }) => {
                    expect(useGameBoardStore().isPlaceable({ x, y }, card)).toBe(false);
                });

                it.each([
                    { x: 1, y: 3 },
                    { x: 5, y: 3 },
                    { x: 3, y: 1 },
                    { x: 3, y: 5 },
                    { x: 3, y: 3 }
                ])('returns false if the card is not adjacent to any tiles, only on top of empty tiles ($x, $y)', ({ x, y }) => {
                    expect(useGameBoardStore().isPlaceable({ x, y }, card3)).toBe(false);
                });

                it.each([
                    { x: 1, y: 1, card: card2 },
                    { x: 4, y: 1, card },
                    { x: 1, y: 3, card },
                    { x: 4, y: 3, card }
                ])('returns false if the card is on top of fill or special tiles ($x, $y)', ({ x, y, card }) => {
                    expect(useGameBoardStore().isPlaceable({ x, y }, card)).toBe(false);
                });
            });

            describe('alpha team', () => {
                beforeEach(() => {
                    // @ts-ignore
                    useRoomStore().playerTeam = PlayerTeam.ALPHA;
                });

                describe.each([true, false])('board flipped for bravo: %s', boardFlipped => {
                    beforeEach(() => {
                        useUserSettingsStore().flipBoardOnBravoTeam = boardFlipped;
                    });

                    it.each([
                        { x: 2, y: 3 },
                        { x: 3, y: 3 }
                    ])('returns false if the card is next to the opposing side\'s tiles ($x, $y)', ({ x, y }) => {
                        expect(useGameBoardStore().isPlaceable({ x, y }, card)).toBe(false);
                    });

                    it.each([
                        { x: 1, y: 1 },
                        { x: 3, y: 1 }
                    ])('returns true if the card is next to the player\'s tiles ($x, $y)', ({ x, y }) => {
                        expect(useGameBoardStore().isPlaceable({ x, y }, card)).toBe(true);
                    });

                    it.each([
                        { x: 2, y: 3 },
                        { x: 3, y: 1 },
                        { x: 3, y: 3 }
                    ])('returns false if doing a special attack next to tiles that aren\'t special squares ($x, $y)', ({ x, y }) => {
                        useCurrentMoveStore().special = true;

                        expect(useGameBoardStore().isPlaceable({ x, y }, card)).toBe(false);
                    });

                    it.each([
                        MST.ACTIVE_SPECIAL_ALPHA,
                        MST.INACTIVE_SPECIAL_ALPHA
                    ])('returns true if doing a special attack next to the player\'s special squares (%#)', specialSquare => {
                        useCurrentMoveStore().special = true;
                        const store = useGameBoardStore();
                        store.board![1][1] = specialSquare;
                        store.board![1][2] = MST.FILL_ALPHA;
                        store.board![2][2] = MST.FILL_BRAVO;

                        expect(store.isPlaceable({ x: 1, y: 1 }, card)).toBe(true);
                    });
                });
            });

            describe('bravo team', () => {
                beforeEach(() => {
                    // @ts-ignore
                    useRoomStore().playerTeam = PlayerTeam.BRAVO;
                });

                describe('board not flipped', () => {
                    beforeEach(() => {
                        useUserSettingsStore().flipBoardOnBravoTeam = false;
                    });

                    it.each([
                        { x: 1, y: 1 },
                        { x: 3, y: 1 }
                    ])('returns false if the card is next to the opposing side\'s tiles ($x, $y)', ({ x, y }) => {
                        expect(useGameBoardStore().isPlaceable({ x, y }, card)).toBe(false);
                    });

                    it.each([
                        { x: 2, y: 3 },
                        { x: 3, y: 3 }
                    ])('returns true if the card is next to the player\'s tiles ($x, $y)', ({ x, y }) => {
                        expect(useGameBoardStore().isPlaceable({ x, y }, card)).toBe(true);
                    });

                    it.each([
                        { x: 1, y: 1 },
                        { x: 3, y: 1 },
                        { x: 3, y: 3 }
                    ])('returns false if doing a special attack next to tiles that aren\'t special squares ($x, $y)', ({ x, y }) => {
                        useCurrentMoveStore().special = true;

                        expect(useGameBoardStore().isPlaceable({ x, y }, card)).toBe(false);
                    });

                    it.each([
                        MST.ACTIVE_SPECIAL_BRAVO,
                        MST.INACTIVE_SPECIAL_BRAVO
                    ])('returns true if doing a special attack next to the player\'s special squares (%#)', specialSquare => {
                        useCurrentMoveStore().special = true;
                        const store = useGameBoardStore();
                        store.board![1][1] = specialSquare;
                        store.board![1][2] = MST.FILL_ALPHA;
                        store.board![2][2] = MST.FILL_BRAVO;

                        expect(store.isPlaceable({ x: 1, y: 1 }, card)).toBe(true);
                    });
                });

                describe('board flipped', () => {
                    beforeEach(() => {
                        useUserSettingsStore().flipBoardOnBravoTeam = true;
                    });

                    it.each([
                        { x: 2, y: 3 },
                        { x: 3, y: 3 }
                    ])('returns false if the card is next to the opposing side\'s tiles ($x, $y)', ({ x, y }) => {
                        expect(useGameBoardStore().isPlaceable({ x, y }, card)).toBe(false);
                    });

                    it.each([
                        { x: 1, y: 1 },
                        { x: 3, y: 1 }
                    ])('returns true if the card is next to the player\'s tiles ($x, $y)', ({ x, y }) => {
                        expect(useGameBoardStore().isPlaceable({ x, y }, card)).toBe(true);
                    });

                    it.each([
                        { x: 3, y: 3 },
                        { x: 2, y: 3 },
                        { x: 2, y: 1 }
                    ])('returns false if doing a special attack next to tiles that aren\'t special squares ($x, $y)', ({ x, y }) => {
                        useCurrentMoveStore().special = true;

                        expect(useGameBoardStore().isPlaceable({ x, y }, card)).toBe(false);
                    });

                    it.each([
                        MST.ACTIVE_SPECIAL_BRAVO,
                        MST.INACTIVE_SPECIAL_BRAVO
                    ])('returns true if doing a special attack next to the player\'s special squares (%#)', specialSquare => {
                        useCurrentMoveStore().special = true;
                        const store = useGameBoardStore();
                        store.board![5][5] = specialSquare;
                        store.board![5][4] = MST.FILL_ALPHA;
                        store.board![4][4] = MST.FILL_BRAVO;

                        expect(store.isPlaceable({ x: 1, y: 1 }, card)).toBe(true);
                    });
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

        describe('specialPointCount', () => {
            it('returns the expected amount of special points', () => {
                const store = useGameBoardStore();
                store.board = [
                    [MST.EMPTY, MST.FILL_BRAVO, MST.ACTIVE_SPECIAL_BRAVO, MST.OUT_OF_BOUNDS],
                    [MST.FILL_ALPHA, MST.INACTIVE_SPECIAL_ALPHA, MST.FILL_ALPHA, MST.FILL_BRAVO],
                    [MST.FILL_BRAVO, MST.ACTIVE_SPECIAL_ALPHA, MST.FILL_BRAVO, MST.FILL_ALPHA],
                    [MST.FILL_BRAVO, MST.FILL_ALPHA, MST.INACTIVE_SPECIAL_BRAVO, MST.EMPTY],
                ];

                expect(store.specialPointCount).toEqual({
                    [PlayerTeam.ALPHA]: 1,
                    [PlayerTeam.BRAVO]: 1
                });
            });
        });
    });

    describe('actions', () => {
        describe('setBoard', () => {
            it('updates the board', () => {
                const store = useGameBoardStore();
                store.usedSpecialPoints = { [PlayerTeam.ALPHA]: 2, [PlayerTeam.BRAVO]: 4 };
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
                expect(store.usedSpecialPoints).toEqual({ [PlayerTeam.ALPHA]: 0, [PlayerTeam.BRAVO]: 0 });
            });

            it.each([
                [PlayerTeam.ALPHA, 2, 1],
                [PlayerTeam.BRAVO, 3, 2]
            ])('updates the position of the active card to the location of the starting square for team %s', (team, expectedX, expectedY) => {
                // @ts-ignore
                useRoomStore().playerTeam = team;
                const activeCardStore = useCurrentMoveStore();
                jest.spyOn(activeCardStore, 'setPositionFromCardOrigin');
                const gameBoardStore = useGameBoardStore();
                const squares = [
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.INACTIVE_SPECIAL_ALPHA, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.INACTIVE_SPECIAL_BRAVO, MST.EMPTY],
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
                (activateSpecialSquares as jest.Mock).mockImplementation(() => {});
            });

            it('applies the chosen moves to the board and activates special tiles', () => {
                const store = useGameBoardStore();

                store.applyMoves({
                    [PlayerTeam.ALPHA]: {
                        type: 'PlaceCard',
                        cardName: 'BombCurling',
                        position: { x: 1, y: 1 },
                        rotation: 270,
                        special: false
                    },
                    [PlayerTeam.BRAVO]: {
                        type: 'PlaceCard',
                        cardName: 'SaberLight00',
                        position: { x: 3, y: 1 },
                        rotation: 0,
                        special: false
                    }
                });

                const expectedBoard = [
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.FILL_ALPHA, MST.INACTIVE_SPECIAL_BRAVO, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.INACTIVE_SPECIAL_ALPHA, MST.FILL_ALPHA, MST.FILL_BRAVO, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.FILL_ALPHA, MST.FILL_BRAVO, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.FILL_BRAVO, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.FILL_BRAVO, MST.EMPTY, MST.EMPTY]
                ];
                expect(activateSpecialSquares).toHaveBeenCalledWith(expectedBoard);
                expect(store.board).toEqual(expectedBoard);
            });

            it('behaves as expected with two identical moves', () => {
                const store = useGameBoardStore();

                store.applyMoves({
                    [PlayerTeam.ALPHA]: {
                        type: 'PlaceCard',
                        cardName: 'BombCurling',
                        position: { x: 1, y: 1 },
                        rotation: 270,
                        special: false
                    },
                    [PlayerTeam.BRAVO]: {
                        type: 'PlaceCard',
                        cardName: 'BombCurling',
                        position: { x: 1, y: 1 },
                        rotation: 270,
                        special: false
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
                        type: 'PlaceCard',
                        cardName: 'BombCurling',
                        position: { x: 2, y: 2 },
                        rotation: 90,
                        special: false
                    },
                    [PlayerTeam.BRAVO]: {
                        type: 'PlaceCard',
                        cardName: 'BombCurling',
                        position: { x: 1, y: 2 },
                        rotation: 180,
                        special: false
                    }
                });

                expect(store.board).toEqual([
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.FILL_BRAVO, MST.NEUTRAL, MST.FILL_BRAVO, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.INACTIVE_SPECIAL_BRAVO, MST.INACTIVE_SPECIAL_ALPHA, MST.EMPTY, MST.EMPTY],
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
                        type: 'PlaceCard',
                        cardName: 'BombCurling',
                        position: { x: 2, y: 2 },
                        rotation: 90,
                        special: false
                    },
                    [PlayerTeam.BRAVO]: {
                        type: 'PlaceCard',
                        cardName: 'BombCurling',
                        position: { x: 1, y: 2 },
                        rotation: 180,
                        special: false
                    }
                });

                expect(store.board).toEqual([
                    [MST.FILL_BRAVO, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.FILL_BRAVO, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.FILL_BRAVO, MST.NEUTRAL, MST.FILL_BRAVO, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.INACTIVE_SPECIAL_BRAVO, MST.INACTIVE_SPECIAL_ALPHA, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.FILL_ALPHA, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY]
                ]);
            });

            it('behaves as expected when overlapping moves have differing costs', () => {
                const store = useGameBoardStore();

                store.applyMoves({
                    [PlayerTeam.ALPHA]: {
                        type: 'PlaceCard',
                        cardName: 'BombCurling',
                        position: { x: 1, y: 1 },
                        rotation: 90,
                        special: false
                    },
                    [PlayerTeam.BRAVO]: {
                        type: 'PlaceCard',
                        cardName: 'SaberLight00',
                        position: { x: 1, y: 1 },
                        rotation: 0,
                        special: false
                    }
                });

                expect(store.board).toEqual([
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.INACTIVE_SPECIAL_BRAVO, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.FILL_ALPHA, MST.INACTIVE_SPECIAL_ALPHA, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.FILL_ALPHA, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.FILL_BRAVO, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.FILL_BRAVO, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY]
                ]);
            });

            it('behaves as expected when overlapping moves have differing costs, regardless of the order turns appear in', () => {
                const store = useGameBoardStore();

                store.applyMoves({
                    [PlayerTeam.BRAVO]: {
                        type: 'PlaceCard',
                        cardName: 'SaberLight00',
                        position: { x: 1, y: 1 },
                        rotation: 0,
                        special: false
                    },
                    [PlayerTeam.ALPHA]: {
                        type: 'PlaceCard',
                        cardName: 'BombCurling',
                        position: { x: 1, y: 1 },
                        rotation: 90,
                        special: false
                    }
                });

                expect(store.board).toEqual([
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.INACTIVE_SPECIAL_BRAVO, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.FILL_ALPHA, MST.INACTIVE_SPECIAL_ALPHA, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.FILL_ALPHA, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.FILL_BRAVO, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.FILL_BRAVO, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY]
                ]);
            });

            it('behaves as expected when special squares overlap in moves with different costs', () => {
                const store = useGameBoardStore();

                store.applyMoves({
                    [PlayerTeam.ALPHA]: {
                        type: 'PlaceCard',
                        cardName: 'BombCurling',
                        position: { x: 0, y: 0 },
                        rotation: 90,
                        special: false
                    },
                    [PlayerTeam.BRAVO]: {
                        type: 'PlaceCard',
                        cardName: 'SaberLight00',
                        position: { x: 1, y: 1 },
                        rotation: 0,
                        special: false
                    }
                });

                expect(store.board).toEqual([
                    [MST.FILL_ALPHA, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.FILL_ALPHA, MST.INACTIVE_SPECIAL_ALPHA, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
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
                        type: 'PlaceCard',
                        cardName: 'SaberLight00',
                        position: { x: 1, y: 1 },
                        rotation: 0,
                        special: false
                    },
                    [PlayerTeam.ALPHA]: {
                        type: 'PlaceCard',
                        cardName: 'BombCurling',
                        position: { x: 0, y: 0 },
                        rotation: 90,
                        special: false
                    }
                });

                expect(store.board).toEqual([
                    [MST.FILL_ALPHA, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.FILL_ALPHA, MST.INACTIVE_SPECIAL_ALPHA, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.FILL_ALPHA, MST.FILL_BRAVO, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.FILL_BRAVO, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.FILL_BRAVO, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.FILL_BRAVO, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY]
                ]);
            });

            it('adds to the used point count for special moves', () => {
                const store = useGameBoardStore();
                store.usedSpecialPoints = { [PlayerTeam.ALPHA]: 2, [PlayerTeam.BRAVO]: 4 };

                store.applyMoves({
                    [PlayerTeam.BRAVO]: {
                        type: 'PlaceCard',
                        cardName: 'SaberLight00',
                        position: { x: 1, y: 1 },
                        rotation: 0,
                        special: true
                    },
                    [PlayerTeam.ALPHA]: {
                        type: 'PlaceCard',
                        cardName: 'BombCurling',
                        position: { x: 0, y: 0 },
                        rotation: 90,
                        special: true
                    }
                });

                expect(store.usedSpecialPoints).toEqual({ [PlayerTeam.ALPHA]: 4, [PlayerTeam.BRAVO]: 6 });
            });
        });
    });
});
