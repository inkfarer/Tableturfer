import { CardRotation } from '~/types/CardRotation';
import { getRotationOffset, withinBoardBounds } from '~/helpers/ActiveCardHelper';
import { useGameBoardStore } from '~/stores/GameBoardStore';
import { MapSquareType as MST } from '~/types/MapSquareType';
import { CardSquareType as CST } from '~/types/CardSquareType';
import { setActivePinia } from 'pinia';
import { createTestingPinia } from '@pinia/testing';

describe('ActiveCardHelper', () => {
    beforeEach(() => {
        setActivePinia(createTestingPinia({
            stubActions: false
        }));
    });

    describe('getRotationOffset', () => {
        it.each([
            [0,   3, 2, 0, 0],
            [90,  3, 2, -1, 1],
            [180, 3, 2, -1, 0],
            [270, 3, 2, -1, 0],
            [0,   7, 5, 0, 0],
            [90,  7, 5, -1, 1],
            [180, 7, 5, -0, 0],
            [270, 7, 5, -1, 1],
            [0,   2, 7, 0, 0],
            [90,  2, 7, 3, -2],
            [180, 2, 7, 0, 1],
            [270, 2, 7, 2, -2],
            [0,   6, 1, 0, 0],
            [90,  6, 1, -2, 3],
            [180, 6, 1, 0, 1],
            [270, 6, 1, -3, 3],
            [0,   5, 1, 0, 0],
            [90,  5, 1, -2, 2],
            [180, 5, 1, -0, 0],
            [270, 5, 1, -2, 2],
            [0,   6, 2, 0, 0],
            [90,  6, 2, -2, 2],
            [180, 6, 2, -0, 0],
            [270, 6, 2, -2, 2],
            [0,   3, 3, 0, 0],
            [90,  3, 3, 0, 0],
            [180, 3, 3, 0, 0],
            [270, 3, 3, 0, 0]
        ])('returns the expected offset [rotation = %d, height = %d, width = %d]', (rotation, height, width, x, y) => {
            expect(getRotationOffset(rotation as CardRotation, { height, width })).toEqual({ x, y });
        });
    });

    describe('withinBoardBounds', () => {
        describe('first card', () => {
            it.each([
                [13, 14, 8, 9, 10, 10],
                [-13, 14, -2, 8, 10, 10],
                [-13, -14, 0, -1, 10, 10],
                [13, -14, 7, -1, 10, 10],
                [15, 18, 4, 7, 6, 8]
            ])(
                'ensures the card is at least partially within the bounds of the board [(%d, %d) turns into (%d, %d) with a %dx%d board]',
                (xStart, yStart, expectedX, expectedY, boardWidth, boardHeight) => {
                    const gameBoardStore = useGameBoardStore();
                    gameBoardStore.board = Array.from({ length: boardHeight }, () => new Array(boardWidth).fill(MST.EMPTY));

                    const result = withinBoardBounds({ x: xStart, y: yStart }, [
                        [CST.EMPTY, CST.FILL, CST.EMPTY],
                        [CST.FILL, CST.SPECIAL, CST.FILL]
                    ]);

                    expect(result).toEqual({ x: expectedX, y: expectedY });
                });
        });

        describe('second card', () => {
            it.each([
                [13, 14, 8, 9, 10, 10],
                [-13, 14, -1, 8, 10, 10],
                [-13, -14, 0, -1, 10, 10],
                [13, -14, 8, -1, 10, 10],
                [15, 18, 4, 7, 6, 8]
            ])(
                'ensures the card is at least partially within the bounds of the board [(%d, %d) turns into (%d, %d) with a %dx%d board]',
                (xStart, yStart, expectedX, expectedY, boardWidth, boardHeight) => {
                    const gameBoardStore = useGameBoardStore();
                    gameBoardStore.board = Array.from({ length: boardHeight }, () => new Array(boardWidth).fill(MST.EMPTY));

                    const result = withinBoardBounds({ x: xStart, y: yStart }, [
                        [CST.FILL, CST.SPECIAL],
                        [CST.FILL, CST.EMPTY]
                    ]);

                    expect(result).toEqual({ x: expectedX, y: expectedY });
                });
        });
    });
});
