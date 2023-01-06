import { MaybeComputedRef } from '@vueuse/shared';
import { computed, ref, useElementSize, useSwipe, watch } from '#imports';
import { useGameBoardStore } from '~/stores/GameBoardStore';
import { useCurrentMoveStore } from '~/stores/CurrentMoveStore';

export default function (target: MaybeComputedRef<(HTMLElement & EventTarget) | null>) {
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

    const gameBoardSwipe = useSwipe(target, {
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
}
