import { Position } from '~/types/Position';

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

export function slice2D<T>(array: Array<Array<T>>, start: Position, end: Position): Array<Array<T>> {
    return array.slice(Math.max(start.y, 0), Math.max(end.y + 1, 0))
        .map(row => row.slice(Math.max(start.x, 0), Math.max(end.x + 1, 0)));
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
