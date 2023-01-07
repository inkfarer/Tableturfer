import { computed, ref, useElementSize, useMouseInElement, useSwipe, watch } from '#imports';
import { useGameBoardStore } from '~/stores/GameBoardStore';
import { useCurrentMoveStore } from '~/stores/CurrentMoveStore';
import { VueInstance } from '@vueuse/core';
import { Ref } from 'vue';

export default function (target: Ref<VueInstance | null>) {
    const gameBoardStore = useGameBoardStore();
    const activeCardStore = useCurrentMoveStore();

    const boardSize = computed(() => gameBoardStore.boardSize);
    const boardElementSize = useElementSize(target);
    const pxToMoveOneSquare = computed(() => Math.min(boardElementSize.width.value, boardElementSize.height.value) / Math.max(boardSize.value.width, boardSize.value.height));

    const amountMoved = ref<{ x: number | null, y: number | null }>({ x: null, y: null });
    watch(amountMoved, (newValue, oldValue) => {
        if (newValue.x !== null && newValue.y !== null) {
            activeCardStore.applyDeltaIfPossible({ x: newValue.x - (oldValue.x ?? 0), y: newValue.y - (oldValue.y ?? 0) });
        }
    });

    const gameBoardSwipe = useSwipe(target as Ref<EventTarget | null>, {
        threshold: 10,
        onSwipe() {
            amountMoved.value = {
                x: -Math.floor(gameBoardSwipe.lengthX.value / pxToMoveOneSquare.value),
                y: -Math.floor(gameBoardSwipe.lengthY.value / pxToMoveOneSquare.value)
            };
        },
        onSwipeEnd() {
            amountMoved.value = { x: null, y: null };
        }
    });

    const mouseInGameBoard = useMouseInElement(target);
    const squareSizePx = computed(() => Math.min(
        mouseInGameBoard.elementHeight.value / boardSize.value.height,
        mouseInGameBoard.elementWidth.value / boardSize.value.width));
    const boardMargin = computed(() => ({
        x: (mouseInGameBoard.elementWidth.value - (squareSizePx.value * boardSize.value.width)) / 2,
        y: (mouseInGameBoard.elementHeight.value - (squareSizePx.value * boardSize.value.height)) / 2
    }));

    watch(() => ({ x: mouseInGameBoard.elementX.value, y: mouseInGameBoard.elementY.value }), newValue => {
        if (mouseInGameBoard.sourceType.value === 'mouse') {
            activeCardStore.setPositionIfPossible({
                x: Math.floor((newValue.x - boardMargin.value.x) / squareSizePx.value),
                y: Math.floor((newValue.y - boardMargin.value.y) / squareSizePx.value)
            }, true);
        }
    });
}
