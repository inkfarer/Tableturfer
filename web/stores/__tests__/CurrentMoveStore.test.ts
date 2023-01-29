import { setActivePinia } from 'pinia';
import { createTestingPinia } from '@pinia/testing';
import { useCurrentMoveStore } from '~/stores/CurrentMoveStore';
import { MapSquareType as MST } from '~/types/MapSquareType';
import { CardRotation } from '~/types/CardRotation';
import { CardSquareType as CST } from '~/types/CardSquareType';
import { CardRarity } from '~/types/CardRarity';
import { useGameBoardStore } from '~/stores/GameBoardStore';
import { getRotationOffset, withinBoardBounds } from '~/helpers/ActiveCardHelper';
import Mock = jest.Mock;

jest.mock('~/helpers/ActiveCardHelper');

describe('CurrentMoveStore', () => {
    beforeEach(() => {
        setActivePinia(createTestingPinia({
            stubActions: false
        }));

        (getRotationOffset as Mock).mockRestore();
    });

    describe('getters', () => {
        describe('cardSizeWithoutRotation', () => {
            it.each([
                [0, 4, 3],
                [90, 3, 4],
                [180, 4, 3],
                [270, 3, 4]
            ])('returns the expected card size when the rotation is %d', (rotation, height, width) => {
                const store = useCurrentMoveStore();
                // @ts-ignore
                store.activeCard = {
                    squares: [
                        [CST.FILL, CST.SPECIAL, CST.FILL],
                        [CST.FILL, CST.FILL, CST.FILL],
                        [CST.FILL, CST.FILL, CST.FILL],
                        [CST.FILL, CST.FILL, CST.EMPTY]
                    ]
                };
                store.rotation = rotation as CardRotation;

                expect(store.cardSizeWithoutRotation).toEqual({ height, width });
            });
        });
    });

    describe('actions', () => {
        describe('setActiveCard', () => {
            beforeEach(() => {
                const gameBoardStore = useGameBoardStore();
                gameBoardStore.board = Array.from({ length: 10 }, () => new Array(10).fill(MST.EMPTY));
            });

            it('handles switching from no card being selected', () => {
                const store = useCurrentMoveStore();
                store.rotation = 90;
                store.activeCard = null;
                store.position = { x: 3, y: 2 };
                // @ts-ignore
                store.cardSizeWithoutRotation = { width: 0, height: 0 };
                (getRotationOffset as Mock)
                    .mockReturnValueOnce({ x: -1, y: 1 })
                    .mockReturnValueOnce({ x: 1, y: 0 });
                (withinBoardBounds as Mock).mockReturnValue({ x: 3, y: 2 });

                store.setActiveCard({
                    category: 'Test',
                    name: 'test card',
                    number: 0,
                    rarity: CardRarity.COMMON,
                    season: 999,
                    specialCost: 10,
                    squares: [
                        [CST.EMPTY, CST.EMPTY, CST.SPECIAL, CST.EMPTY, CST.EMPTY],
                        [CST.FILL, CST.FILL, CST.FILL, CST.FILL, CST.FILL]
                    ]
                });

                const expectedSquares = [
                    [CST.EMPTY, CST.EMPTY, CST.SPECIAL, CST.EMPTY, CST.EMPTY],
                    [CST.FILL, CST.FILL, CST.FILL, CST.FILL, CST.FILL]
                ];
                expect(store.rotation).toBe(0);
                expect(store.activeCard).toEqual({
                    category: 'Test',
                    name: 'test card',
                    number: 0,
                    rarity: CardRarity.COMMON,
                    season: 999,
                    specialCost: 10,
                    origin: {
                        x: 2,
                        y: 1
                    },
                    squares: expectedSquares
                });
                expect(store.position).toEqual({ x: 3, y: 2 });
                expect(getRotationOffset).toHaveBeenCalledWith(90, { width: 0, height: 0 });
                expect(getRotationOffset).toHaveBeenCalledWith(0, { width: 5, height: 2 });
                expect(withinBoardBounds).toHaveBeenCalledWith({ x: 3, y: 0 }, expectedSquares);
            });

            it('handles switching between cards', () => {
                const store = useCurrentMoveStore();
                // @ts-ignore
                store.activeCard = {
                    origin: { x: 1, y: 3 }
                };
                store.position = { x: 4, y: 2 };
                store.rotation = 180;
                // @ts-ignore
                store.cardSizeWithoutRotation = { width: 3, height: 5 };
                (getRotationOffset as Mock)
                    .mockReturnValueOnce({ x: -1, y: 1 })
                    .mockReturnValueOnce({ x: 1, y: 0 });
                (withinBoardBounds as Mock).mockReturnValue({ x: 3, y: 2 });

                store.setActiveCard({
                    category: 'Test 2',
                    name: 'test card 2',
                    number: -1,
                    rarity: CardRarity.FRESH,
                    season: 998,
                    specialCost: 11,
                    squares: [
                        [CST.EMPTY, CST.FILL, CST.EMPTY, CST.FILL, CST.EMPTY],
                        [CST.FILL, CST.FILL, CST.FILL, CST.FILL, CST.FILL],
                        [CST.FILL, CST.EMPTY, CST.FILL, CST.EMPTY, CST.FILL],
                        [CST.FILL, CST.FILL, CST.EMPTY, CST.SPECIAL, CST.FILL]
                    ]
                });

                const expectedSquares = [
                    [CST.EMPTY, CST.FILL, CST.EMPTY, CST.FILL, CST.EMPTY],
                    [CST.FILL, CST.FILL, CST.FILL, CST.FILL, CST.FILL],
                    [CST.FILL, CST.EMPTY, CST.FILL, CST.EMPTY, CST.FILL],
                    [CST.FILL, CST.FILL, CST.EMPTY, CST.SPECIAL, CST.FILL]
                ];
                expect(store.rotation).toBe(0);
                expect(store.activeCard).toEqual({
                    category: 'Test 2',
                    name: 'test card 2',
                    number: -1,
                    rarity: CardRarity.FRESH,
                    season: 998,
                    specialCost: 11,
                    origin: {
                        x: 2,
                        y: 2
                    },
                    squares: expectedSquares
                });
                expect(store.position).toEqual({ x: 3, y: 2 });
                expect(getRotationOffset).toHaveBeenCalledWith(180, { width: 3, height: 5 });
                expect(getRotationOffset).toHaveBeenCalledWith(0, { width: 5, height: 4 });
                expect(withinBoardBounds).toHaveBeenCalledWith({ x: 5, y: 2 }, expectedSquares);
            });

            it('handles clearing the currently selected card', () => {
                const store = useCurrentMoveStore();
                // @ts-ignore
                store.activeCard = {
                    origin: { x: 2, y: 1 }
                };
                store.position = { x: 4, y: 2 };
                store.rotation = 180;
                // @ts-ignore
                store.cardSizeWithoutRotation = { width: 3, height: 5 };
                (getRotationOffset as Mock)
                    .mockReturnValueOnce({ x: -1, y: 1 })
                    .mockReturnValueOnce({ x: 1, y: 0 });
                (withinBoardBounds as Mock).mockReturnValue({ x: 3, y: 2 });

                store.setActiveCard(null);

                expect(store.activeCard).toBeNull();
            });
        });

        describe('nextRotationStep', () => {
            it.each([
                [0, 90],
                [90, 180],
                [180, 270],
                [270, 0]
            ])('rotates the card from %d degrees to %d degrees', (originalRotation, expectedRotation) => {
                const store = useCurrentMoveStore();
                // @ts-ignore
                store.activeCard = {
                    squares: [
                        [CST.SPECIAL, CST.EMPTY],
                        [CST.FILL, CST.FILL]
                    ]
                };
                store.rotation = originalRotation as CardRotation;
                store.position = { x: 2, y: 3 };
                // @ts-ignore
                store.cardSizeWithoutRotation = { width: 3, height: 4 };
                (getRotationOffset as Mock)
                    .mockReturnValueOnce({ x: -1, y: 1 })
                    .mockReturnValueOnce({ x: 1, y: 0 });
                (withinBoardBounds as Mock).mockReturnValue({ x: 3, y: 1 });

                store.nextRotationStep();

                const expectedSquares = [
                    [CST.FILL, CST.SPECIAL],
                    [CST.FILL, CST.EMPTY]
                ];
                expect(getRotationOffset).toHaveBeenCalledWith(originalRotation, { width: 3, height: 4 });
                expect(getRotationOffset).toHaveBeenCalledWith(expectedRotation, { width: 3, height: 4 });
                expect(withinBoardBounds).toHaveBeenCalledWith({ x: 4, y: 2 }, expectedSquares);
                expect(store.position).toEqual({ x: 3, y: 1 });
                expect(store.rotation).toBe(expectedRotation);
                expect(store.activeCard).toEqual({
                    squares: expectedSquares
                });
            });

            it('does nothing if no card is active', () => {
                const store = useCurrentMoveStore();
                store.activeCard = null;
                store.rotation = 0;
                store.position = { x: 0, y: 0 };

                store.nextRotationStep();

                expect(store.position).toEqual({ x: 0, y: 0 });
                expect(store.rotation).toBe(0);
                expect(store.activeCard).toBeNull();
                expect(getRotationOffset).not.toHaveBeenCalled();
            });
        });

        describe('previousRotationStep', () => {
            it.each([
                [0, 270],
                [90, 0],
                [180, 90],
                [270, 180]
            ])('rotates the card from %d degrees to %d degrees', (originalRotation, expectedRotation) => {
                const store = useCurrentMoveStore();
                // @ts-ignore
                store.activeCard = {
                    squares: [
                        [CST.SPECIAL, CST.EMPTY],
                        [CST.FILL, CST.FILL]
                    ]
                };
                store.rotation = originalRotation as CardRotation;
                store.position = { x: 2, y: 3 };
                // @ts-ignore
                store.cardSizeWithoutRotation = { width: 4, height: 3 };
                (getRotationOffset as Mock)
                    .mockReturnValueOnce({ x: -2, y: 2 })
                    .mockReturnValueOnce({ x: 2, y: 1 });
                (withinBoardBounds as Mock).mockReturnValue({ x: 5, y: 1 });

                store.previousRotationStep();

                const expectedSquares = [
                    [CST.EMPTY, CST.FILL],
                    [CST.SPECIAL, CST.FILL]
                ];
                expect(getRotationOffset).toHaveBeenCalledWith(originalRotation, { width: 4, height: 3 });
                expect(getRotationOffset).toHaveBeenCalledWith(expectedRotation, { width: 4, height: 3 });
                expect(withinBoardBounds).toHaveBeenCalledWith({ x: 6, y: 2 }, expectedSquares);
                expect(store.position).toEqual({ x: 5, y: 1 });
                expect(store.rotation).toBe(expectedRotation);
                expect(store.activeCard).toEqual({
                    squares: expectedSquares
                });
            });

            it('does nothing if no card is active', () => {
                const store = useCurrentMoveStore();
                store.activeCard = null;
                store.rotation = 0;
                store.position = { x: 0, y: 0 };

                store.previousRotationStep();

                expect(store.position).toEqual({ x: 0, y: 0 });
                expect(store.rotation).toBe(0);
                expect(store.activeCard).toBeNull();
                expect(getRotationOffset).not.toHaveBeenCalled();
            });
        });

        describe('setPositionFromCardOrigin', () => {
            it('updates the card\'s position, accounting for its origin point and rotation', () => {
                const store = useCurrentMoveStore();
                // @ts-ignore
                store.activeCard = {
                    origin: {
                        x: 3,
                        y: 2
                    }
                };
                store.position = {
                    x: 0,
                    y: 0
                };
                // @ts-ignore
                store.cardSizeWithoutRotation = { width: 2, height: 3 };
                store.rotation = 90;
                (getRotationOffset as Mock).mockReturnValue({ x: 1, y: 2 });

                store.setPositionFromCardOrigin({
                    x: 5,
                    y: 3
                });

                expect(store.position).toEqual({
                    x: 3,
                    y: 3
                });
                expect(getRotationOffset).toHaveBeenCalledWith(90, { width: 2, height: 3 });
            });

            it('updates the position when no card is selected', () => {
                const store = useCurrentMoveStore();
                // @ts-ignore
                store.activeCard = null;
                store.position = {
                    x: 0,
                    y: 0
                };
                // @ts-ignore
                store.cardSizeWithoutRotation = { width: 0, height: 0 };
                store.rotation = 180;
                (getRotationOffset as Mock).mockReturnValue({ x: 4, y: -1 });

                store.setPositionFromCardOrigin({
                    x: 5,
                    y: 3
                });

                expect(store.position).toEqual({
                    x: 9,
                    y: 2
                });
                expect(getRotationOffset).toHaveBeenCalledWith(180, { width: 0, height: 0 });
            });
        });

        describe('applyDeltaIfPossible', () => {
            it('changes the position if no squares of the active card are moved on top of disabled squares', () => {
                const gameBoardStore = useGameBoardStore();
                gameBoardStore.board = [
                    [MST.DISABLED, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY]
                ];
                const currentMoveStore = useCurrentMoveStore();
                // @ts-ignore
                currentMoveStore.activeCard = {
                    squares: [
                        [CST.FILL, CST.FILL],
                        [CST.SPECIAL, CST.FILL]
                    ]
                };
                currentMoveStore.position = { x: 1, y: 0 };

                currentMoveStore.applyDeltaIfPossible({ x: -1, y: 0 });

                expect(currentMoveStore.position).toEqual({ x: 0, y: 0 });
            });

            it('does not change the position when trying to move the card out of bounds', () => {
                const gameBoardStore = useGameBoardStore();
                gameBoardStore.board = [
                    [MST.DISABLED, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY]
                ];
                const currentMoveStore = useCurrentMoveStore();
                // @ts-ignore
                currentMoveStore.activeCard = {
                    squares: [
                        [CST.FILL, CST.FILL],
                        [CST.SPECIAL, CST.EMPTY]
                    ]
                };
                currentMoveStore.position = { x: 0, y: 0 };

                currentMoveStore.applyDeltaIfPossible({ x: -1, y: 0 });

                expect(currentMoveStore.position).toEqual({ x: 0, y: 0 });
            });

            it('allows moving cards outside bounds as long as no new squares are moved outside the play area', () => {
                const gameBoardStore = useGameBoardStore();
                gameBoardStore.board = [
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY],
                    [MST.EMPTY, MST.EMPTY, MST.EMPTY]
                ];
                const currentMoveStore = useCurrentMoveStore();
                // @ts-ignore
                currentMoveStore.activeCard = {
                    squares: [
                        [CST.FILL, CST.FILL, CST.FILL],
                        [CST.EMPTY, CST.EMPTY, CST.FILL],
                        [CST.EMPTY, CST.EMPTY, CST.FILL],
                        [CST.FILL, CST.FILL, CST.FILL]
                    ]
                };
                currentMoveStore.position = { x: -2, y: 0 };

                currentMoveStore.applyDeltaIfPossible({ x: 0, y: 1 });
                expect(currentMoveStore.position).toEqual({ x: -2, y: 1 });

                currentMoveStore.applyDeltaIfPossible({ x: 0, y: 1 });
                expect(currentMoveStore.position).toEqual({ x: -2, y: 2 });

                currentMoveStore.applyDeltaIfPossible({ x: 0, y: 1 });
                expect(currentMoveStore.position).toEqual({ x: -2, y: 2 });

                currentMoveStore.applyDeltaIfPossible({ x: -1, y: 0 });
                expect(currentMoveStore.position).toEqual({ x: -2, y: 2 });

                currentMoveStore.applyDeltaIfPossible({ x: 1, y: 0 });
                expect(currentMoveStore.position).toEqual({ x: -1, y: 2 });
            });
        });

        describe('moveUp', () => {
            it('moves the card upward', () => {
                const store = useCurrentMoveStore();
                jest.spyOn(store, 'applyDeltaIfPossible').mockReturnValue();

                store.moveUp();

                expect(store.applyDeltaIfPossible).toHaveBeenCalledWith({ x: 0, y: -1 });
            });
        });

        describe('moveDown', () => {
            it('moves the card downward', () => {
                const store = useCurrentMoveStore();
                jest.spyOn(store, 'applyDeltaIfPossible').mockReturnValue();

                store.moveDown();

                expect(store.applyDeltaIfPossible).toHaveBeenCalledWith({ x: 0, y: 1 });
            });
        });

        describe('moveLeft', () => {
            it('moves the card to the left', () => {
                const store = useCurrentMoveStore();
                jest.spyOn(store, 'applyDeltaIfPossible').mockReturnValue();

                store.moveLeft();

                expect(store.applyDeltaIfPossible).toHaveBeenCalledWith({ x: -1, y: 0 });
            });
        });

        describe('moveRight', () => {
            it('moves the card to the right', () => {
                const store = useCurrentMoveStore();
                jest.spyOn(store, 'applyDeltaIfPossible').mockReturnValue();

                store.moveRight();

                expect(store.applyDeltaIfPossible).toHaveBeenCalledWith({ x: 1, y: 0 });
            });
        });
    });
});

