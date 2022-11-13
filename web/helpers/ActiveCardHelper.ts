import { CardSize } from '~/stores/ActiveCardStore';
import { CardRotation } from '~/types/CardRotation';
import { Position } from '~/types/Position';
import { CardSquareType } from '~/types/CardSquareType';
import { useGameBoardStore } from '~/stores/GameBoardStore';
import { some2D } from '~/helpers/ArrayHelper';
import { MapSquareType } from '~/types/MapSquareType';

// Strictly matches the way the card moves as it is rotated in-game. Can probably be cleaned up.
export function getRotationOffset(rotation: CardRotation, cardSizeWithoutRotation: CardSize): Position {
    if (rotation === 0) {
        return { x: 0, y: 0 };
    } else {
        const { width, height } = cardSizeWithoutRotation;
        if (width === height) {
            return { x: 0, y: 0 };
        }

        switch (rotation) {
            case 90: {
                let x = Math.ceil((width - height) / 2);
                const y = Math.ceil((height - width) / 2);

                if (height % 2 === 1 && width % 2 === 0) {
                    x -= 1;
                }

                return { x, y };
            }
            case 180: {
                if (height % 2 === 0 && width % 2 === 1) {
                    return { x: 0, y: (height + width) % 2 };
                } else {
                    return { x: ((height + width) % 2) * -1, y: 0 };
                }
            }
            case 270: {
                const x = Math.floor((width - height) / 2);
                let y = x * -1;

                if (height % 2 === 1 && width % 2 === 0) {
                    y -= 1;
                }

                return { x, y };
            }
        }
    }
}

// Attempt to nudge the card back into bounds if required
// Likely quite inefficient but acceptable for now
export function withinBoardBounds(position: Position, squares?: CardSquareType[][] | null): Position {
    if (squares == null) {
        return position;
    }

    const cardWidth = squares[0]?.length ?? 0;
    const cardHeight = squares.length;
    const gameBoardStore = useGameBoardStore();
    const newPosition = { ...position };

    if (
        (cardHeight === 0 && cardWidth === 0)
        || !gameBoardStore.cardIsOutOfBounds(newPosition, { width: cardWidth, height: cardHeight })
    ) {
        return position;
    }

    const someSquaresWithinBounds = (): boolean => {
        const squaresUnderPosition = gameBoardStore.boardSquaresUnderCard(
            newPosition,
            { width: cardWidth, height: cardHeight });
        return some2D(squaresUnderPosition, (boardSquare, position) => {
            const cardSquare = squares[position.y][position.x];

            return cardSquare !== CardSquareType.EMPTY && boardSquare !== MapSquareType.OUT_OF_BOUNDS;
        });
    };

    while (!someSquaresWithinBounds()) {
        if (newPosition.x < 0) {
            newPosition.x++;
        } else if (newPosition.x + cardWidth - 1 >= gameBoardStore.boardSize.width) {
            newPosition.x--;
        }

        if (!someSquaresWithinBounds()) {
            if (newPosition.y < 0) {
                newPosition.y++;
            } else if (newPosition.y + cardHeight - 1 >= gameBoardStore.boardSize.height) {
                newPosition.y--;
            }
        } else {
            break;
        }
    }

    return newPosition;
}
