import { computed, ref, useElementBounding, usePointer, useSwipe, watch } from '#imports';
import { useGameBoardStore } from '~/stores/GameBoardStore';
import { useCurrentMoveStore } from '~/stores/CurrentMoveStore';
import { VueInstance } from '@vueuse/core';
import { Ref } from 'vue';

export default function (target: Ref<VueInstance | null>) {
    const gameBoardStore = useGameBoardStore();
    const activeCardStore = useCurrentMoveStore();

    const boardSize = computed(() => gameBoardStore.boardSize);
    const boardElementSize = useElementBounding(target);
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

    const mouseInGameBoard = usePointer({ target: target as Ref<EventTarget | null>, pointerTypes: ['mouse']});
    const squareSizePx = computed(() => Math.min(
        boardElementSize.height.value / boardSize.value.height,
        boardElementSize.width.value / boardSize.value.width));
    const boardMargin = computed(() => ({
        x: (boardElementSize.width.value - (squareSizePx.value * boardSize.value.width)) / 2,
        y: (boardElementSize.height.value - (squareSizePx.value * boardSize.value.height)) / 2
    }));
    const mouseBoardPosition = computed(() => ({
        x: Math.floor((mouseInGameBoard.x.value - boardElementSize.left.value - boardMargin.value.x) / squareSizePx.value),
        y: Math.floor((mouseInGameBoard.y.value - boardElementSize.top.value - boardMargin.value.y) / squareSizePx.value)
    }));

    watch(mouseBoardPosition, (newValue, oldValue) => {
        if (oldValue != null && newValue.x === oldValue.x && newValue.y === oldValue.y) {
            return;
        }
        activeCardStore.setPositionInsideBoard(newValue);
    });
}
