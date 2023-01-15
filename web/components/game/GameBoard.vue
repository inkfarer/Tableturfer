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
import { computed, onMounted, onUnmounted, ref, watchEffect } from '#imports';
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

    const fillGhostPatternCanvas = document.createElement('canvas');
    const specialGhostPatternCanvas = document.createElement('canvas');

    function drawFillGhostPattern(canvas: HTMLCanvasElement, playerTeam: PlayerTeam) {
        const patternSize = 50;
        canvas.width = patternSize;
        canvas.height = patternSize;

        const ctx = canvas.getContext('2d');
        if (ctx == null) {
            throw new Error('Failed to access canvas drawing context');
        }

        // Goes wild with uneven numbers, ok for now
        const lineCount = 6;
        const lineSize = (patternSize / lineCount);
        const lineOffset = lineSize / 2;
        ctx.globalAlpha = 0.9;
        ctx.fillStyle = getFillSquareColor(playerTeam);

        ctx.beginPath();
        ctx.moveTo(0, 0);
        ctx.lineTo(lineSize / 2, 0);
        ctx.lineTo(0, lineSize / 2);

        for (let i = 1; i <= lineCount; i += 1) {
            ctx.moveTo(0, lineSize * (i * 2) - lineOffset);
            ctx.lineTo(0, lineSize * (i * 2 + 1) - lineOffset);
            ctx.lineTo(lineSize * (i * 2 + 1) - lineOffset, 0);
            ctx.lineTo(lineSize * (i * 2) - lineOffset, 0);
        }

        ctx.fill();
    }

    function drawSpecialGhostPattern(canvas: HTMLCanvasElement, playerTeam: PlayerTeam) {
        const patternSize = 50;
        canvas.width = patternSize;
        canvas.height = patternSize;

        const ctx = canvas.getContext('2d');
        if (ctx == null) {
            throw new Error('Failed to access canvas drawing context');
        }

        ctx.globalAlpha = 0.9;
        ctx.fillStyle = getSpecialSquareColor(playerTeam);
        const gridSize = 4;
        const dotSpacing = patternSize / gridSize;
        const dotSize = 3;

        for (let x = 0; x < gridSize + 1; x++) {
            for (let y = 0; y < gridSize + 1; y++) {
                ctx.beginPath();
                ctx.arc(dotSpacing * x, dotSpacing * y, dotSize, 0, 2 * Math.PI);
                ctx.fill();
            }
        }
    }

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
            ctx.globalAlpha = 1;
            const fillPath = new Path2D();
            const specialPath = new Path2D();
            forEach2D(activeCard, (item, position) => {
                if (item === CardSquareType.EMPTY) {
                    return;
                }

                const x = squareSize * (position.x + activeCardPosition.x) + offsetX;
                const y = squareSize * (position.y + activeCardPosition.y) + offsetY;

                if (item === CardSquareType.FILL) {
                    fillPath.rect(x, y, squareSize, squareSize);
                } else if (item === CardSquareType.SPECIAL) {
                    specialPath.rect(x, y, squareSize, squareSize);
                } else {
                    console.warn(`I don't know how to create a ghost for square type ${item}.`);
                }
            });

            ctx.save();
            ctx.clip(specialPath);
            drawSpecialGhostPattern(specialGhostPatternCanvas, playerTeam);
            // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
            const specialPattern = ctx.createPattern(specialGhostPatternCanvas, null)!;
            specialPattern.setTransform(new DOMMatrixReadOnly().rotate(30));
            ctx.fillStyle = specialPattern;
            ctx.translate(offsetX, offsetY);
            const specialPatternScale = squareSize / specialGhostPatternCanvas.width;
            ctx.scale(specialPatternScale, specialPatternScale);
            ctx.fillRect(0, 0, width / specialPatternScale, height / specialPatternScale);
            ctx.restore();

            ctx.save();
            ctx.clip(fillPath);
            drawFillGhostPattern(fillGhostPatternCanvas, playerTeam);
            // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
            ctx.fillStyle = ctx.createPattern(fillGhostPatternCanvas, null)!;
            ctx.translate(offsetX, offsetY);
            const patternScale = squareSize / fillGhostPatternCanvas.width;
            ctx.scale(patternScale, patternScale);
            ctx.fillRect(0, 0, width / patternScale, height / patternScale);
            ctx.restore();
        }
    }

    function getFillSquareColor(team: PlayerTeam): string {
        if (placeable.value) {
            return team === PlayerTeam.ALPHA ? '#EBF800' : '#495CFF';
        } else {
            return '#AAA';
        }
    }

    function getSpecialSquareColor(team: PlayerTeam): string {
        if (placeable.value) {
            return team === PlayerTeam.ALPHA ? '#FFA100' : '#08F0FF';
        } else {
            return '#AAA';
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

    watchEffect(() => {
        redraw(
            canvas,
            gameBoardStore.board,
            activeCardStore.activeCard?.squares ?? null,
            activeCardStore.position,
            roomStore.playerTeam,
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
