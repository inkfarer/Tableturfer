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
