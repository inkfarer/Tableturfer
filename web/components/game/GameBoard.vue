<template>
    <div
        class="board"
        :class="{
            'special-attack': activeCardStore.special
        }"
    >
        <canvas
            ref="gameBoardCanvas"
            class="game-board-canvas"
        />
    </div>
</template>

<script lang="ts" setup>
import { useGameBoardStore } from '~/stores/GameBoardStore';
import { useCurrentMoveStore } from '~/stores/CurrentMoveStore';
import { computed, onMounted, ref, watch } from '#imports';
import { forEach2D, getSize } from '~/helpers/ArrayHelper';
import { MapSquareType } from '~/types/MapSquareType';
import { CardSquareType } from '~/types/CardSquareType';
import { Position } from '~/types/Position';
import { PlayerTeam } from '~/types/PlayerTeam';
import { useRoomStore } from '~/stores/RoomStore';

const activeCardStore = useCurrentMoveStore();
const gameBoardStore = useGameBoardStore();
const roomStore = useRoomStore();
const gameBoardCanvas = ref<HTMLCanvasElement | null>(null);

const placeable = computed(() => {
    if (activeCardStore.activeCard == null) {
        return false;
    }

    return gameBoardStore.isPlaceable(activeCardStore.position, activeCardStore.activeCard.squares);
});

function redraw(
    canvas: HTMLCanvasElement,
    board: MapSquareType[][] | null,
    activeCard: CardSquareType[][] | null,
    activeCardPosition: Position,
    playerTeam: PlayerTeam | null,
    specialActive: boolean,
    passing: boolean
) {
    const ctx = canvas.getContext('2d'),
        width = canvas.clientWidth,
        height = canvas.clientHeight;
    if (ctx == null) {
        throw new Error('Failed to access canvas drawing context');
    }

    canvas.width = width;
    canvas.height = height;

    if (board == null) {
        return;
    }

    const strokeSize = 1;
    const boardSize = getSize(board);
    const squareSize = Math.floor(Math.min((height - strokeSize * 2) / boardSize.height, width / boardSize.width));
    const offset = Math.floor(Math.abs(width - height) / 2);
    const offsetX = width > height ? offset : 0;
    const offsetY = width < height ? offset : 0;

    forEach2D(board, (item, position) => {
        if (item === MapSquareType.DISABLED) {
            return;
        }

        const x = squareSize * position.x + offsetX;
        const y = squareSize * position.y + offsetY;

        ctx.globalAlpha = 1;
        ctx.fillStyle = getMapFill(MapSquareType.EMPTY);
        ctx.strokeStyle = '#393939';
        ctx.lineWidth = strokeSize;
        ctx.fillRect(x, y, squareSize, squareSize);
        ctx.strokeRect(x + (strokeSize / 2), y + (strokeSize / 2), squareSize - strokeSize, squareSize - strokeSize);

        if (item !== MapSquareType.EMPTY) {
            if (specialActive && (item === MapSquareType.FILL_ALPHA || item === MapSquareType.FILL_BRAVO)) {
                ctx.globalAlpha = 0.2;
            }

            ctx.fillStyle = getMapFill(item);
            ctx.fillRect(x, y, squareSize, squareSize);
        }
    });

    if (!passing && activeCard != null && playerTeam != null) {
        forEach2D(activeCard, (item, position) => {
            if (item === CardSquareType.EMPTY) {
                return;
            }

            const x = squareSize * (position.x + activeCardPosition.x) + offsetX;
            const y = squareSize * (position.y + activeCardPosition.y) + offsetY;

            ctx.fillStyle = getCardFill(item, playerTeam);
            ctx.fillRect(x, y, squareSize, squareSize);
        });
    }
}

function getCardFill(square: CardSquareType, team: PlayerTeam): string {
    if (placeable.value) {
        switch (square) {
            case CardSquareType.FILL:
                return team === PlayerTeam.ALPHA ? 'rgba(236, 144, 9, 0.5)' : 'rgba(75, 80, 243, 0.2)';
            case CardSquareType.SPECIAL:
                return team === PlayerTeam.ALPHA ? 'rgba(236, 144, 9, 0.8)' : 'rgba(21, 227, 219, 0.5)';
            default:
                return 'transparent';
        }
    } else {
        switch (square) {
            case CardSquareType.FILL:
                return 'rgba(255, 255, 255, 0.2)';
            case CardSquareType.SPECIAL:
                return 'rgba(255, 255, 255, 0.5)';
            default:
                return 'transparent';
        }
    }
}

function getMapFill(square: MapSquareType): string {
    switch (square) {
        case MapSquareType.ACTIVE_SPECIAL_ALPHA:
            return '#A4FFFB';
        case MapSquareType.ACTIVE_SPECIAL_BRAVO:
            return '#FFBC5A';
        case MapSquareType.FILL_ALPHA:
            return '#E9FF0F';
        case MapSquareType.FILL_BRAVO:
            return '#4B50F3';
        case MapSquareType.INACTIVE_SPECIAL_ALPHA:
            return '#EC9009';
        case MapSquareType.INACTIVE_SPECIAL_BRAVO:
            return '#15E3DB';
        case MapSquareType.NEUTRAL:
            return '#aaa';
        case MapSquareType.EMPTY:
            return '#171717';
        default:
            return 'transparent';
    }
}

onMounted(() => {
    const canvas = gameBoardCanvas.value;
    if (canvas == null) {
        throw new Error('GameBoard is missing canvas');
    }

    new ResizeObserver(() =>
        redraw(
            canvas,
            gameBoardStore.board,
            activeCardStore.activeCard?.squares ?? null,
            activeCardStore.position,
            roomStore.playerTeam,
            activeCardStore.special,
            activeCardStore.pass)
    ).observe(canvas);

    watch(() => [activeCardStore.activeCard?.squares, activeCardStore.position, activeCardStore.special, activeCardStore.pass] as [CardSquareType[][], Position, boolean, boolean],
        ([newSquares, newPosition, special, pass]) => {
            redraw(
                canvas,
                gameBoardStore.board,
                newSquares,
                newPosition,
                roomStore.playerTeam,
                special,
                pass);
        });
    watch(() => gameBoardStore.board, newValue => {
        redraw(
            canvas,
            newValue,
            activeCardStore.activeCard?.squares ?? null,
            activeCardStore.position,
            roomStore.playerTeam,
            activeCardStore.special,
            activeCardStore.pass);
    });
    watch(() => roomStore.playerTeam, newValue => {
        redraw(
            canvas,
            gameBoardStore.board,
            activeCardStore.activeCard?.squares ?? null,
            activeCardStore.position, newValue,
            activeCardStore.special,
            activeCardStore.pass);
    });
});
</script>

<style lang="scss">
.board {
    position: relative;
    height: max-content;
}

.game-board-canvas {
    width: 100%;
    height: 100%;
}
</style>
