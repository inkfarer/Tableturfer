import { MapSquareType as MST } from '~/types/MapSquareType';
import { activateSpecialSquares } from '~/helpers/BoardHelper';

describe('BoardHelper', () => {
    describe('activateSpecialSquares', () => {
        describe.each([
            ['alpha', MST.INACTIVE_SPECIAL_ALPHA, MST.ACTIVE_SPECIAL_ALPHA],
            ['bravo', MST.INACTIVE_SPECIAL_BRAVO, MST.ACTIVE_SPECIAL_BRAVO]
        ])('team %s', (teamName, inactiveSquare, activeSquare) => {
            it('converts inactive special squares that may be activated to active ones', () => {
                const board = [
                    [MST.DISABLED, MST.OUT_OF_BOUNDS, MST.FILL_BRAVO],
                    [MST.FILL_ALPHA, inactiveSquare, MST.INACTIVE_SPECIAL_ALPHA],
                    [MST.FILL_ALPHA, inactiveSquare, MST.NEUTRAL],
                    [MST.INACTIVE_SPECIAL_BRAVO, MST.ACTIVE_SPECIAL_ALPHA, MST.ACTIVE_SPECIAL_BRAVO]
                ];

                activateSpecialSquares(board);

                expect(board).toEqual([
                    [MST.DISABLED, MST.OUT_OF_BOUNDS, MST.FILL_BRAVO],
                    [MST.FILL_ALPHA, activeSquare, MST.ACTIVE_SPECIAL_ALPHA],
                    [MST.FILL_ALPHA, activeSquare, MST.NEUTRAL],
                    [MST.ACTIVE_SPECIAL_BRAVO, MST.ACTIVE_SPECIAL_ALPHA, MST.ACTIVE_SPECIAL_BRAVO]
                ]);
            });

            it('does not convert special squares that are near empty squares', () => {
                const board = [
                    [MST.DISABLED, MST.OUT_OF_BOUNDS, MST.FILL_BRAVO],
                    [MST.FILL_ALPHA, inactiveSquare, MST.INACTIVE_SPECIAL_ALPHA],
                    [MST.FILL_ALPHA, inactiveSquare, MST.NEUTRAL],
                    [MST.INACTIVE_SPECIAL_BRAVO, MST.ACTIVE_SPECIAL_ALPHA, MST.EMPTY]
                ];

                activateSpecialSquares(board);

                expect(board).toEqual([
                    [MST.DISABLED, MST.OUT_OF_BOUNDS, MST.FILL_BRAVO],
                    [MST.FILL_ALPHA, activeSquare, MST.ACTIVE_SPECIAL_ALPHA],
                    [MST.FILL_ALPHA, inactiveSquare, MST.NEUTRAL],
                    [MST.ACTIVE_SPECIAL_BRAVO, MST.ACTIVE_SPECIAL_ALPHA, MST.EMPTY]
                ]);
            });
        });
    });
});
