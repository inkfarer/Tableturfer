<template>
    <div
        class="card-placement-overlay"
        :style="{
            transform: `translate(${Constants.BOARD_SQUARE_SIZE_PX * activeCardStore.offsetPosition.x}px, ${Constants.BOARD_SQUARE_SIZE_PX * activeCardStore.offsetPosition.y}px)`
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
import { useActiveCardStore } from '~/stores/ActiveCardStore';

const activeCardStore = useActiveCardStore();
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
