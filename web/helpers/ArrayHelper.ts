import { Position } from '~/types/Position';

// todo: consider creating a "Matrix" class containing all these functions

export function rotateClockwise<T>(array: Array<Array<T>>): Array<Array<T>> {
    return array[0].map((val, index) => array.map(row => row[index]).reverse());
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
