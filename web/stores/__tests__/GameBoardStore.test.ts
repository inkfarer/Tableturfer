import { setActivePinia } from 'pinia';
import { MapSquareType as MST } from '~/types/MapSquareType';
import { CardSquareType as CST } from '~/types/CardSquareType';
import { useGameBoardStore } from '~/stores/GameBoardStore';
import { useActiveCardStore } from '~/stores/ActiveCardStore';
import { createTestingPinia } from '@pinia/testing';
import { PlayerTeam } from '~/types/PlayerTeam';
import { useRoomStore } from '~/stores/RoomStore';

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
                    expect(useGameBoardStore().isPlaceable({ x, y }, card)).toBe(false);
                });

                it.each([
                    { x: 2, y: 3 },
                    { x: 3, y: 3 }
                ])('returns true if the card is next to the player\'s tiles ($x, $y)', ({ x, y }) => {
                    expect(useGameBoardStore().isPlaceable({ x, y }, card)).toBe(true);
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

        describe('placeCard', () => {
            it('does nothing if the card cannot be placed', () => {
                const store = useGameBoardStore();
                const isPlaceable = jest.fn().mockReturnValue(false);
                // @ts-ignore
                store.isPlaceable = isPlaceable;
                store.board = [
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY]
                ];

                store.placeCard({ x: 0, y: 0 }, [
                    [CST.FILL, CST.FILL]
                ], PlayerTeam.ALPHA);

                expect(store.board).toEqual([
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY]
                ]);
                expect(isPlaceable).toHaveBeenCalledWith({ x: 0, y: 0 }, [
                    [CST.FILL, CST.FILL]
                ]);
            });

            it('places the card on the board for the alpha team', () => {
                const store = useGameBoardStore();
                const isPlaceable = jest.fn().mockReturnValue(true);
                // @ts-ignore
                store.isPlaceable = isPlaceable;
                store.board = [
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY]
                ];

                store.placeCard({ x: 1, y: 0 }, [
                    [CST.FILL, CST.SPECIAL],
                    [CST.EMPTY, CST.FILL]
                ], PlayerTeam.ALPHA);

                expect(store.board).toEqual([
                    [MST.EMPTY, MST.FILL_ALPHA, MST.SPECIAL_ALPHA],
                    [MST.EMPTY, MST.EMPTY, MST.FILL_ALPHA],
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY]
                ]);
                expect(isPlaceable).toHaveBeenCalledWith({ x: 1, y: 0 }, [
                    [CST.FILL, CST.SPECIAL],
                    [CST.EMPTY, CST.FILL]
                ]);
            });

            it('places the card on the board for the bravo team', () => {
                const store = useGameBoardStore();
                const isPlaceable = jest.fn().mockReturnValue(true);
                // @ts-ignore
                store.isPlaceable = isPlaceable;
                store.board = [
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY]
                ];

                store.placeCard({ x: 1, y: 1 }, [
                    [CST.SPECIAL, CST.FILL],
                    [CST.FILL, CST.EMPTY]
                ], PlayerTeam.BRAVO);

                expect(store.board).toEqual([
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.SPECIAL_BRAVO, MST.FILL_BRAVO],
                    [MST.EMPTY, MST.FILL_BRAVO, MST.EMPTY]
                ]);
                expect(isPlaceable).toHaveBeenCalledWith({ x: 1, y: 1 }, [
                    [CST.SPECIAL, CST.FILL],
                    [CST.FILL, CST.EMPTY]
                ]);
            });
        });
    });
});
