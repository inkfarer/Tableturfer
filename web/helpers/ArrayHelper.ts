import { Position } from '~/types/Position';
import { CardSquareType } from '~/types/CardSquareType';
import Constants from '~/data/Constants';
import chunk from 'lodash/chunk';

// todo: consider creating a "Matrix" class containing all these functions

export function rotateClockwise<T>(array: Array<Array<T>>): Array<Array<T>> {
    return array[0].map((val, index) => array.map(row => row[index]).reverse());
}

export function rotateClockwiseBy<T>(array: Array<Array<T>>, degrees: 0 | 90 | 180 | 270): Array<Array<T>> {
    switch (degrees) {
        case 0:
            return array;
        case 90:
            return rotateClockwise(array);
        case 180:
            return array.map(row => row.reverse()).reverse();
        case 270:
            return array[0].map((val, index) => array.map(row => row[index])).reverse();
    }
}

export function rotateCounterclockwise<T>(array: Array<Array<T>>): Array<Array<T>> {
    return array[0].map((val, index) => array.map(row => row[row.length - 1 - index]));
}

export function findIndex2D<T>(array: Array<Array<T>>, predicate: (item: T) => boolean): Position | null {
    for (let y = 0; y < array.length; y++) {
        const x = array[y].findIndex(predicate);

        if (x > 0) {
            return { x, y };
        }
    }

    return null;
}

export function slice2D<T>(array: Array<Array<T>>, start: Position, end: Position, placeholder?: T): Array<Array<T>> {
    const slicedArray = array.slice(Math.max(start.y, 0), Math.max(end.y + 1, 0))
        .map(row => row.slice(Math.max(start.x, 0), Math.max(end.x + 1, 0)));

    const expectedHeight = Math.abs(start.y - end.y) + 1;
    const expectedWidth = Math.abs(start.x - end.x) + 1;
    if (placeholder !== undefined && (expectedWidth !== slicedArray[0]?.length || expectedHeight !== slicedArray.length)) {
        const resultWidth = slicedArray[0]?.length ?? 0;

        // Add missing rows to the top and bottom
        if (start.y < 0) {
            const rowsToAddToTop = Math.abs(start.y - Math.min(0, end.y));

            slicedArray.unshift(...Array.from({ length: rowsToAddToTop }, () => new Array(resultWidth).fill(placeholder)));
        }
        const currentHeight = slicedArray.length;
        if (expectedHeight > currentHeight) {
            slicedArray.push(...Array.from({ length: expectedHeight - currentHeight }, () => new Array(resultWidth).fill(placeholder)));
        }

        // Add missing rows to the left and right
        if (start.x < 0) {
            const columnsToAddToLeft = Math.abs(start.x - Math.min(0, end.x));

            slicedArray.forEach(row => row.unshift(...new Array(columnsToAddToLeft).fill(placeholder)));
        }
        const currentWidth = slicedArray[0]?.length;
        if (expectedWidth > currentWidth) {
            slicedArray.forEach(row => {
                row.push(...new Array(expectedWidth - currentWidth).fill(placeholder));
            });
        }
    }

    return slicedArray;
}

export function some2D<T>(array: Array<Array<T>>, predicate: (item: T, position: Position) => boolean): boolean {
    for (let y = 0; y < array.length; y++) {
        for (let x = 0; x < array[y].length; x++) {
            if (predicate(array[y][x], { x, y })) {
                return true;
            }
        }
    }

    return false;
}

export function every2D<T>(array: Array<Array<T>>, predicate: (item: T, position: Position) => boolean): boolean {
    for (let y = 0; y < array.length; y++) {
        for (let x = 0; x < array[y].length; x++) {
            if (!predicate(array[y][x], { x, y })) {
                return false;
            }
        }
    }

    return true;
}

export function forEach2D<T>(array: Array<Array<T>>, callback: (item: T, position: Position) => unknown) {
    for (let y = 0; y < array.length; y++) {
        for (let x = 0; x < array[y].length; x++) {
            callback(array[y][x], { x, y });
        }
    }
}

export function count2D<T>(array: T[][], predicate: (item: T, position: Position) => boolean): number {
    let count = 0;

    for (let y = 0; y < array.length; y++) {
        for (let x = 0; x < array[y].length; x++) {
            if (predicate(array[y][x], { x, y })) {
                count++;
            }
        }
    }

    return count;
}

export function fill2D<T>(width: number, height: number, value: T): T[][] {
    return Array.from({ length: height }, () => Array.from({ length: width }, () => value));
}

// Removes empty rows and columns
// todo: this should be done when we import cards into a database
export function normalizeCardSquares(squares: CardSquareType[]): CardSquareType[][] {
    const emptyColumns = new Set();
    for (let i = 0; i < Constants.CARD_GRID_SIZE; i++) {
        if (squares
            .filter((square, squareIndex) => squareIndex % Constants.CARD_GRID_SIZE === i)
            .every(square => square === CardSquareType.EMPTY)
        ) {
            emptyColumns.add(i);
        }
    }

    return chunk(squares, Constants.CARD_GRID_SIZE)
        .filter(row => row.some(square => square !== CardSquareType.EMPTY))
        .map(row => row.filter((square, index) => !emptyColumns.has(index)))
        .reverse();
}
