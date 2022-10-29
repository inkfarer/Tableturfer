import { setActivePinia } from 'pinia';
import { createTestingPinia } from '@pinia/testing';
import { useActiveCardStore } from '~/stores/ActiveCardStore';
import { CardRotation } from '~/types/CardRotation';
import { CardSquareType as CST } from '~/types/CardSquareType';
import { CardRarity } from '~/types/CardRarity';

describe('ActiveCardStore', () => {
    beforeEach(() => {
        setActivePinia(createTestingPinia({
            stubActions: false
        }));
    });

    describe('getters', () => {
        describe('cardSize', () => {
            it.each([
                [0, 4, 3],
                [90, 3, 4],
                [180, 4, 3],
                [270, 3, 4]
            ])('returns the expected card size when the rotation is %d', (rotation, height, width) => {
                const store = useActiveCardStore();
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

                expect(store.cardSize).toEqual({ height, width });
            });
        });

        describe('offsetPosition', () => {
            it.each([
                [0,   3, 2, 2, 3],
                [90,  3, 2, 1, 4],
                [180, 3, 2, 1, 3],
                [270, 3, 2, 1, 3],
                [0,   7, 5, 2, 3],
                [90,  7, 5, 1, 4],
                [180, 7, 5, 2, 3],
                [270, 7, 5, 1, 4],
                [0,   2, 7, 2, 3],
                [90,  2, 7, 5, 1],
                [180, 2, 7, 2, 4],
                [270, 2, 7, 4, 1],
                [0,   6, 1, 2, 3],
                [90,  6, 1, 0, 6],
                [180, 6, 1, 2, 4],
                [270, 6, 1, -1, 6],
                [0,   5, 1, 2, 3],
                [90,  5, 1, 0, 5],
                [180, 5, 1, 2, 3],
                [270, 5, 1, 0, 5],
                [0,   6, 2, 2, 3],
                [90,  6, 2, 0, 5],
                [180, 6, 2, 2, 3],
                [270, 6, 2, 0, 5],
                [0,   3, 3, 2, 3],
                [90,  3, 3, 2, 3],
                [180, 3, 3, 2, 3],
                [270, 3, 3, 2, 3]
            ])('returns the expected position [rotation = %d, height = %d, width = %d]', (rotation, height, width, x, y) => {
                const store = useActiveCardStore();
                store.position = { x: 2, y: 3 };
                store.rotation = rotation as CardRotation;
                // @ts-ignore
                store.cardSize = { width, height };

                expect(store.offsetPosition).toEqual({ x, y });
            });
        });
    });

    describe('actions', () => {
        describe('setActiveCard', () => {
            it('handles switching from no card being selected', () => {
                const store = useActiveCardStore();
                store.rotation = 90;
                store.activeCard = null;
                store.position = { x: 3, y: 2 };

                store.setActiveCard({
                    rowId: 'testCard',
                    category: 'Test',
                    name: 'test card',
                    number: 0,
                    rarity: CardRarity.COMMON,
                    season: 999,
                    specialCost: 10,
                    squares: [
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.FILL,
                        CST.FILL,
                        CST.FILL,
                        CST.FILL,
                        CST.FILL,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.SPECIAL,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY
                    ]
                });

                expect(store.rotation).toBe(0);
                expect(store.activeCard).toEqual({
                    rowId: 'testCard',
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
                    squares: [
                        [CST.EMPTY, CST.EMPTY, CST.SPECIAL, CST.EMPTY, CST.EMPTY],
                        [CST.FILL, CST.FILL, CST.FILL, CST.FILL, CST.FILL]
                    ]
                });
                expect(store.position).toEqual({ x: 1, y: 1 });
            });

            it('handles switching between cards', () => {
                const store = useActiveCardStore();
                // @ts-ignore
                store.activeCard = {
                    origin: { x: 1, y: 3 }
                };
                store.position = { x: 4, y: 2 };
                store.rotation = 180;

                store.setActiveCard({
                    rowId: 'testCard2',
                    category: 'Test 2',
                    name: 'test card 2',
                    number: -1,
                    rarity: CardRarity.FRESH,
                    season: 998,
                    specialCost: 11,
                    squares: [
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.FILL,
                        CST.FILL,
                        CST.EMPTY,
                        CST.SPECIAL,
                        CST.FILL,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.FILL,
                        CST.EMPTY,
                        CST.FILL,
                        CST.EMPTY,
                        CST.FILL,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.FILL,
                        CST.FILL,
                        CST.FILL,
                        CST.FILL,
                        CST.FILL,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.FILL,
                        CST.EMPTY,
                        CST.FILL,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY,
                        CST.EMPTY
                    ]
                });

                expect(store.rotation).toBe(0);
                expect(store.activeCard).toEqual({
                    rowId: 'testCard2',
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
                    squares: [
                        [CST.EMPTY, CST.FILL, CST.EMPTY, CST.FILL, CST.EMPTY],
                        [CST.FILL, CST.FILL, CST.FILL, CST.FILL, CST.FILL],
                        [CST.FILL, CST.EMPTY, CST.FILL, CST.EMPTY, CST.FILL],
                        [CST.FILL, CST.FILL, CST.EMPTY, CST.SPECIAL, CST.FILL]
                    ]
                });
                expect(store.position).toEqual({ x: 3, y: 3 });
            });

            it('handles clearing the currently selected card', () => {
                const store = useActiveCardStore();
                // @ts-ignore
                store.activeCard = {
                    origin: { x: 2, y: 1 }
                };
                store.position = { x: 4, y: 2 };
                store.rotation = 180;

                store.setActiveCard(null);

                expect(store.activeCard).toBeNull();
                expect(store.rotation).toBe(0);
                expect(store.position).toEqual({ x: 6, y: 3 });
            });
        });

        describe('nextRotationStep', () => {
            it.each([
                [0, 90],
                [90, 180],
                [180, 270],
                [270, 0]
            ])('rotates the card from %d degrees to %d degrees', (originalRotation, expectedRotation) => {
                const store = useActiveCardStore();
                // @ts-ignore
                store.activeCard = {
                    squares: [
                        [CST.SPECIAL, CST.EMPTY],
                        [CST.FILL, CST.FILL]
                    ]
                };
                store.rotation = originalRotation as CardRotation;

                store.nextRotationStep();

                expect(store.rotation).toBe(expectedRotation);
                expect(store.activeCard).toEqual({
                    squares: [
                        [CST.FILL, CST.SPECIAL],
                        [CST.FILL, CST.EMPTY]
                    ]
                });
            });

            it('does nothing if no card is active', () => {
                const store = useActiveCardStore();
                store.activeCard = null;
                store.rotation = 0;

                store.nextRotationStep();

                expect(store.rotation).toBe(0);
                expect(store.activeCard).toBeNull();
            });
        });

        describe('previousRotationStep', () => {
            it.each([
                [0, 270],
                [90, 0],
                [180, 90],
                [270, 180]
            ])('rotates the card from %d degrees to %d degrees', (originalRotation, expectedRotation) => {
                const store = useActiveCardStore();
                // @ts-ignore
                store.activeCard = {
                    squares: [
                        [CST.SPECIAL, CST.EMPTY],
                        [CST.FILL, CST.FILL]
                    ]
                };
                store.rotation = originalRotation as CardRotation;

                store.previousRotationStep();

                expect(store.rotation).toBe(expectedRotation);
                expect(store.activeCard).toEqual({
                    squares: [
                        [CST.EMPTY, CST.FILL],
                        [CST.SPECIAL, CST.FILL]
                    ]
                });
            });

            it('does nothing if no card is active', () => {
                const store = useActiveCardStore();
                store.activeCard = null;
                store.rotation = 0;

                store.previousRotationStep();

                expect(store.rotation).toBe(0);
                expect(store.activeCard).toBeNull();
            });
        });

        describe('setPosition', () => {
            it('updates the card\'s position, accounting for its origin point', () => {
                const store = useActiveCardStore();
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

                store.setPosition({
                    x: 5,
                    y: 3
                });

                expect(store.position).toEqual({
                    x: 2,
                    y: 1
                });
            });

            it('updates the position when no card is selected', () => {
                const store = useActiveCardStore();
                // @ts-ignore
                store.activeCard = null;
                store.position = {
                    x: 0,
                    y: 0
                };

                store.setPosition({
                    x: 5,
                    y: 3
                });

                expect(store.position).toEqual({
                    x: 5,
                    y: 3
                });
            });
        });

        describe('moveUp', () => {
            it('moves the card upward', () => {
                const store = useActiveCardStore();
                store.position = { x: 4, y: 3 };

                store.moveUp();

                expect(store.position).toEqual({ x: 4, y: 2 });
            });
        });

        describe('moveDown', () => {
            it('moves the card downward', () => {
                const store = useActiveCardStore();
                store.position = { x: 4, y: 6 };

                store.moveDown();

                expect(store.position).toEqual({ x: 4, y: 7 });
            });
        });

        describe('moveLeft', () => {
            it('moves the card to the left', () => {
                const store = useActiveCardStore();
                store.position = { x: 4, y: 6 };

                store.moveLeft();

                expect(store.position).toEqual({ x: 3, y: 6 });
            });
        });

        describe('moveRight', () => {
            it('moves the card to the right', () => {
                const store = useActiveCardStore();
                store.position = { x: 8, y: 1 };

                store.moveRight();

                expect(store.position).toEqual({ x: 9, y: 1 });
            });
        });
    });
});

