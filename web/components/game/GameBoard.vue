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
import { computed, onMounted, onUnmounted, ref, watch } from '#imports';
import { forEach2D, getSize } from '~/helpers/ArrayHelper';
import { MapSquareType } from '~/types/MapSquareType';
import { CardSquareType } from '~/types/CardSquareType';
import { Position } from '~/types/Position';
import { PlayerTeam } from '~/types/PlayerTeam';
import { useRoomStore } from '~/stores/RoomStore';
import { createImage } from '~/utils/ImageUtil';

const activeCardStore = useCurrentMoveStore();
const gameBoardStore = useGameBoardStore();
const roomStore = useRoomStore();
const gameBoardCanvas = ref<HTMLCanvasElement | null>(null);
const resizeObserver = ref<ResizeObserver | null>(null);

const placeable = computed(() => {
    if (activeCardStore.activeCard == null) {
        return false;
    }

    return gameBoardStore.isPlaceable(activeCardStore.position, activeCardStore.activeCard.squares);
});

onMounted(async () => {
    const imgFillAlpha = await createImage('/img/squares/1x/fill-alpha.webp');
    const imgFillBravo = await createImage('/img/squares/1x/fill-bravo.webp');
    const imgSpecialAlpha = await createImage('/img/squares/1x/special-alpha.webp');
    const imgSpecialAlphaActive = await createImage('/img/squares/1x/special-alpha-active.webp');
    const imgSpecialBravo = await createImage('/img/squares/1x/special-bravo.webp');
    const imgSpecialBravoActive = await createImage('/img/squares/1x/special-bravo-active.webp');
    const imgNeutral = await createImage('/img/squares/1x/neutral.webp');

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
            pixelRatio = window.devicePixelRatio,
            width = canvas.clientWidth,
            height = canvas.clientHeight;
        const dpiWidth = Math.round(pixelRatio * width);
        const dpiHeight = Math.round(pixelRatio * height);
        if (ctx == null) {
            throw new Error('Failed to access canvas drawing context');
        }

        canvas.width = dpiWidth;
        canvas.height = dpiHeight;
        ctx.scale(pixelRatio, pixelRatio);

        if (board == null) {
            return;
        }

        const strokeSize = 1;
        const boardSize = getSize(board);
        const squareSize = Math.min((height - strokeSize * 2) / boardSize.height, width / boardSize.width);
        const boardSizePx = {
            width: boardSize.width * squareSize,
            height: boardSize.height * squareSize
        };
        const offsetX = (width - boardSizePx.width) / 2;
        const offsetY = (height - boardSizePx.height) / 2;

        forEach2D(board, (item, position) => {
            if (item === MapSquareType.DISABLED) {
                return;
            }

            const x = squareSize * position.x + offsetX;
            const y = squareSize * position.y + offsetY;

            ctx.globalAlpha = 1;
            ctx.fillStyle = '#171717';
            ctx.strokeStyle = '#393939';
            ctx.lineWidth = strokeSize;
            ctx.fillRect(x, y, squareSize, squareSize);
            ctx.strokeRect(x + (strokeSize / 2), y + (strokeSize / 2), squareSize - strokeSize, squareSize - strokeSize);
        });

        forEach2D(board, (item, position) => {
            if (item === MapSquareType.DISABLED || item === MapSquareType.EMPTY) {
                return;
            }

            const img = getSquareSprite(item);
            if (img == null) {
                return;
            }

            ctx.globalAlpha = 1;
            const x = squareSize * position.x + offsetX;
            const y = squareSize * position.y + offsetY;

            if (specialActive && (item === MapSquareType.FILL_ALPHA || item === MapSquareType.FILL_BRAVO)) {
                ctx.globalAlpha = 0.2;
            }

            ctx.drawImage(img, x, y, squareSize, squareSize);
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

    function getSquareSprite(square: MapSquareType): HTMLImageElement | null {
        switch (square) {
            case MapSquareType.FILL_ALPHA:
                return imgFillAlpha;
            case MapSquareType.FILL_BRAVO:
                return imgFillBravo;
            case MapSquareType.ACTIVE_SPECIAL_ALPHA:
                return imgSpecialAlphaActive;
            case MapSquareType.INACTIVE_SPECIAL_ALPHA:
                return imgSpecialAlpha;
            case MapSquareType.ACTIVE_SPECIAL_BRAVO:
                return imgSpecialBravoActive;
            case MapSquareType.INACTIVE_SPECIAL_BRAVO:
                return imgSpecialBravo;
            case MapSquareType.NEUTRAL:
                return imgNeutral;
            default:
                return null;
        }
    }

    const canvas = gameBoardCanvas.value;
    if (canvas == null) {
        throw new Error('GameBoard is missing canvas');
    }

    if (resizeObserver.value) {
        resizeObserver.value.disconnect();
    }
    resizeObserver.value = new ResizeObserver(() =>
        redraw(
            canvas,
            gameBoardStore.board,
            activeCardStore.activeCard?.squares ?? null,
            activeCardStore.position,
            roomStore.playerTeam,
            activeCardStore.special,
            activeCardStore.pass));
    resizeObserver.value.observe(canvas);

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

onUnmounted(() => {
    if (resizeObserver.value) {
        resizeObserver.value.disconnect();
    }
});
</script>

<style lang="scss">
.board {
    position: relative;
}

.game-board-canvas {
    box-sizing: border-box;
    position: absolute;
    width: 100%;
    height: 100%;
}
</style>
