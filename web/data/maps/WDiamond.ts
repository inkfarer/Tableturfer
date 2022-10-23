import { GameMap } from '~/types/GameMap';
import { MapSquareType as MST } from '~/types/MapSquareType';

export const WDiamond: GameMap = {
    name: 'WDiamond',
    squares: [
        [MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.EMPTY, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED],
        [MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED],
        [MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED],
        [MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED],
        [MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED],
        [MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.SPECIAL_BRAVO, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.DISABLED, MST.DISABLED, MST.DISABLED],
        [MST.DISABLED, MST.DISABLED, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.DISABLED, MST.DISABLED],
        [MST.DISABLED, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.DISABLED],
        [MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
        [MST.DISABLED, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.DISABLED],
        [MST.DISABLED, MST.DISABLED, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.DISABLED, MST.DISABLED],
        [MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.DISABLED, MST.DISABLED, MST.DISABLED],
        [MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED],
        [MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.DISABLED, MST.DISABLED, MST.DISABLED],
        [MST.DISABLED, MST.DISABLED, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.DISABLED, MST.DISABLED],
        [MST.DISABLED, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.DISABLED],
        [MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY],
        [MST.DISABLED, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.DISABLED],
        [MST.DISABLED, MST.DISABLED, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.DISABLED, MST.DISABLED],
        [MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.SPECIAL_ALPHA, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.DISABLED, MST.DISABLED, MST.DISABLED],
        [MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED],
        [MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED],
        [MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED],
        [MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.EMPTY, MST.EMPTY, MST.EMPTY, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED],
        [MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.EMPTY, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED, MST.DISABLED],
    ]
};
