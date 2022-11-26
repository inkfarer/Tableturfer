import { MapSquareType } from '~/types/MapSquareType';
import { every2D, forEach2D, slice2D } from '~/helpers/ArrayHelper';
import { isInactiveSpecialSquare } from '~/helpers/SquareHelper';

export function activateSpecialSquares(board: MapSquareType[][]) {
    forEach2D(board, (square, position) => {
        if (!isInactiveSpecialSquare(square)) {
            return;
        }

        const squaresAround = slice2D<MapSquareType>(
            board,
            { x: position.x - 1, y: position.y - 1 },
            { x: position.x + 1, y: position.y + 1 });

        if (every2D(squaresAround, square => square !== MapSquareType.EMPTY)) {
            if (square === MapSquareType.INACTIVE_SPECIAL_ALPHA) {
                board[position.y][position.x] = MapSquareType.ACTIVE_SPECIAL_ALPHA;
            } else if (square === MapSquareType.INACTIVE_SPECIAL_BRAVO) {
                board[position.y][position.x] = MapSquareType.ACTIVE_SPECIAL_BRAVO;
            }
        }
    });
}
