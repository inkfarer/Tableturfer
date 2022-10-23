<template>
    <div
        class="card-placement-overlay"
        :style="{
            transform: `translate(${Constants.BOARD_SQUARE_SIZE_PX * normalizedPosition.x}px, ${Constants.BOARD_SQUARE_SIZE_PX * normalizedPosition.y}px)`
        }"
    >
        <div
            v-for="(row, rowIndex) in activeCardStore.activeCard?.squares ?? []"
            :key="`row_${rowIndex}`"
            class="square-row"
        >
            <div
                v-for="(square, squareIndex) in row"
                :key="`square_${rowIndex}_${squareIndex}`"
                :class="`card-square_${square}`"
                class="square"
                :style="{
                    height: `${Constants.BOARD_SQUARE_SIZE_PX}px`,
                    width: `${Constants.BOARD_SQUARE_SIZE_PX}px`
                }"
            />
        </div>
    </div>
</template>

<script lang="ts" setup>
import Constants from '~/data/Constants';
import { Position } from '~/types/Position';
import { useActiveCardStore } from '~/stores/ActiveCardStore';
import { computed } from '#imports';

const activeCardStore = useActiveCardStore();

const normalizedPosition = computed(() => {
    const rotation = activeCardStore.rotation;
    if (rotation === 0) {
        return activeCardStore.position;
    } else {
        const { width, height } = activeCardStore.cardSize;
        if (width === height) {
            return activeCardStore.position;
        }

        const addToPosition = (x: number, y: number): Position => ({
            x: activeCardStore.position.x + x,
            y: activeCardStore.position.y + y
        });

        switch (rotation) {
            case 90: {
                let x = Math.ceil((width - height) / 2);
                const y = Math.ceil((height - width) / 2);

                if (height % 2 === 1 && width % 2 === 0) {
                    x -= 1;
                }

                return addToPosition(x, y);
            }
            case 180: {
                if (height % 2 === 0 && width % 2 === 1) {
                    return addToPosition(0, (height + width) % 2);
                } else {
                    return addToPosition(((height + width) % 2) * -1, 0);
                }
            }
            case 270: {
                const x = Math.floor((width - height) / 2);
                let y = x * -1;

                if (height % 2 === 1 && width % 2 === 0) {
                    y -= 1;
                }

                return addToPosition(x, y);
            }
        }
    }
});
</script>

<style lang="scss">
.card-placement-overlay {
    position: absolute;
}

.square {
    &.card-square_1 {
        background-color: rgba(236, 144, 9, 0.5);
    }

    &.card-square_2 {
        background-color: rgba(236, 144, 9, 0.8);
    }
}
</style>
