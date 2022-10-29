import { every2D, findIndex2D, rotateClockwise, rotateCounterclockwise, slice2D, some2D } from '~/helpers/ArrayHelper';

describe('ArrayHelper', () => {
    describe('slice2D', () => {
        const input = [
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16]
        ];

        it.each([
            {
                x1: 0, y1: 0, x2: 1, y2: 1,
                result: [
                    [1, 2],
                    [5, 6]
                ]
            },
            {
                x1: 1, y1: 0, x2: 2, y2: 1,
                result: [
                    [2, 3],
                    [6, 7]
                ]
            },
            {
                x1: 1, y1: 1, x2: 1, y2: 1,
                result: [
                    [6]
                ]
            },
            {
                x1: 2, y1: 1, x2: 3, y2: 3,
                result: [
                    [7, 8],
                    [11, 12],
                    [15, 16]
                ]
            }
        ])('returns a partial slice of the given array [start: ($x1, $y1), end: ($x2, $y2)]', ({ x1, y1, x2, y2, result }) => {
            expect(slice2D(input, { x: x1, y: y1 }, { x: x2, y: y2 })).toEqual(result);
        });

        it.each([
            {
                x1: -1, y1: -2, x2: 1, y2: 1,
                result: [
                    [1, 2],
                    [5, 6]
                ]
            },
            {
                x1: -2, y1: -3, x2: -1, y2: -1,
                result: []
            }
        ])('returns the expected result when given negative coordinates [start: ($x1, $y1), end: ($x2, $y2)]', ({ x1, y1, x2, y2, result }) => {
            expect(slice2D(input, { x: x1, y: y1 }, { x: x2, y: y2 })).toEqual(result);
        });

        it.each([
            {
                x1: 2, y1: 2, x2: 10, y2: 9,
                result: [
                    [11, 12],
                    [15, 16]
                ]
            },
            {
                x1: 20, y1: 10, x2: 21, y2: 12,
                result: []
            }
        ])('returns the expected result when the input coordinates exceed the size of the input array [start: ($x1, $y1), end: ($x2, $y2)]', ({ x1, y1, x2, y2, result }) => {
            expect(slice2D(input, { x: x1, y: y1 }, { x: x2, y: y2 })).toEqual(result);
        });

        it.each([
            {
                x1: 2, y1: 2, x2: 4, y2: 5,
                result: [
                    [11, 12, -1],
                    [15, 16, -1],
                    [-1, -1, -1],
                    [-1, -1, -1]
                ]
            },
            {
                x1: -2, y1: 2, x2: 1, y2: 5,
                result: [
                    [-1, -1, 9, 10],
                    [-1, -1, 13, 14],
                    [-1, -1, -1, -1],
                    [-1, -1, -1, -1]
                ]
            },
            {
                x1: -2, y1: -1, x2: 1, y2: 1,
                result: [
                    [-1, -1, -1, -1],
                    [-1, -1, 1, 2],
                    [-1, -1, 5, 6]
                ]
            },
            {
                x1: 2, y1: -2, x2: 5, y2: 1,
                result: [
                    [-1, -1, -1, -1],
                    [-1, -1, -1, -1],
                    [3, 4, -1, -1],
                    [7, 8, -1, -1]
                ]
            },
            {
                x1: -1, y1: -1, x2: 4, y2: 4,
                result: [
                    [-1, -1, -1, -1, -1, -1],
                    [-1, 1, 2, 3, 4, -1],
                    [-1, 5, 6, 7, 8, -1],
                    [-1, 9, 10, 11, 12, -1],
                    [-1, 13, 14, 15, 16, -1],
                    [-1, -1, -1, -1, -1, -1]
                ]
            },
            {
                x1: 20, y1: 10, x2: 21, y2: 12,
                result: [
                    [-1, -1],
                    [-1, -1],
                    [-1, -1]
                ]
            }
        ])('returns the expected result when the input coordinates exceed the size of the input array and a placeholder item is given [start: ($x1, $y1), end: ($x2, $y2)]', ({ x1, y1, x2, y2, result }) => {
            expect(slice2D(input, { x: x1, y: y1 }, { x: x2, y: y2 }, -1)).toEqual(result);
        });

        it('handles the placeholder value being null', () => {
            expect(slice2D(input, { x: 2, y: 2 }, { x: 4, y: 4 }, null)).toEqual([
                [11, 12, null],
                [15, 16, null],
                [null, null, null]
            ]);
        });

        it.each([
            { x1: 10, y1: 11, x2: 1, y2: 1 },
            { x1: 2, y1: 1, x2: 1, y2: 0 }
        ])('returns the expected result if the start position is larger than the end position [start: ($x1, $y1), end: ($x2, $y2)]', ({ x1, y1, x2, y2 }) => {
            expect(slice2D(input, { x: x1, y: y1 }, { x: x2, y: y2 })).toEqual([]);
        });
    });

    describe('some2D', () => {
        it('returns true if any items in the matrix match the predicate', () => {
            expect(some2D([
                [1, 2, 3],
                [4, 5, 6],
                [7, 8, 9]
            ], (item, position) => item === 4 && position.x === 0 && position.y === 1)).toEqual(true);
        });

        it('returns false if none of the items in the matrix match the predicate', () => {
            expect(some2D([
                [1, 2, 3],
                [4, 5, 6],
                [7, 8, 9]
            ], item => item === 14)).toEqual(false);
        });
    });

    describe('every2D', () => {
        it('returns true if every item in the matrix matches the predicate', () => {
            expect(every2D([
                [2, 4, 6],
                [8, 10, 12],
                [14, 16, 18]
            ], item => item % 2 === 0)).toEqual(true);
        });

        it('provides the expected positions to the predicate function', () => {
            expect.assertions(8);

            every2D([
                ['a', 'b'],
                ['c', 'd'],
            ], (item, position) => {
                expect(position.x).toEqual({
                    a: 0,
                    b: 1,
                    c: 0,
                    d: 1
                }[item]);
                expect(position.y).toEqual({
                    a: 0,
                    b: 0,
                    c: 1,
                    d: 1
                }[item]);

                return true;
            });
        });

        it('returns false if some of the items in the matrix don\'t match the predicate', () => {
            expect(every2D([
                [2, 4, 6],
                [8, 10, 12],
                [14, 16, 19]
            ], item => item % 2 === 0)).toEqual(false);
        });
    });

    describe('findIndex2D', () => {
        it('returns the position of the first item in the matrix matching the predicate', () => {
            expect(findIndex2D([
                [1, 2, 3],
                [4, 5, 6],
                [1, 2, 3]
            ], item => item === 2)).toEqual({ x: 1, y: 0 });
        });

        it('returns null if no item is found', () => {
            expect(findIndex2D([
                [1, 2, 3],
                [4, 5, 6],
                [1, 2, 3]
            ], item => item === 9)).toBeNull();
        });
    });

    describe('rotateClockwise', () => {
        it('rotates the input array', () => {
            expect(rotateClockwise([
                [1, 2, 3],
                [4, 5, 6]
            ])).toEqual([
                [4, 1],
                [5, 2],
                [6, 3]
            ]);
        });
    });

    describe('rotateCounterclockwise', () => {
        it('rotates the input array', () => {
            expect(rotateCounterclockwise([
                [1, 2, 3],
                [4, 5, 6]
            ])).toEqual([
                [3, 6],
                [2, 5],
                [1, 4]
            ]);
        });
    });
});
