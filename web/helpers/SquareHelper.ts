import { CardSquareType } from '~/types/CardSquareType';
import { MapSquareType } from '~/types/MapSquareType';
import { PlayerTeam } from '~/types/PlayerTeam';

export function mapSquareFromCardSquare(square: CardSquareType, team: PlayerTeam): MapSquareType {
    switch (square) {
        case CardSquareType.EMPTY:
            return MapSquareType.EMPTY;
        case CardSquareType.FILL:
            if (team === PlayerTeam.ALPHA) {
                return MapSquareType.FILL_ALPHA;
            } else {
                return MapSquareType.FILL_BRAVO;
            }
        case CardSquareType.SPECIAL:
            if (team === PlayerTeam.ALPHA) {
                return MapSquareType.SPECIAL_ALPHA;
            } else {
                return MapSquareType.SPECIAL_BRAVO;
            }
    }
}

export function isFillSquare(square: MapSquareType): boolean {
    return square === MapSquareType.FILL_ALPHA || square === MapSquareType.FILL_BRAVO;
}

export function isSpecialSquare(square: MapSquareType): boolean {
    return square === MapSquareType.SPECIAL_ALPHA || square === MapSquareType.SPECIAL_BRAVO;
}
