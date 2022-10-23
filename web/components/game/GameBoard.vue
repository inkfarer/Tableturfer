<template>
    <div class="board">
        <GameCardPlacementOverlay />
        <div
            v-for="(row, rowIndex) in gameBoardStore.board"
            :key="`row_${rowIndex}`"
            class="square-row"
        >
            <div
                v-for="(square, squareIndex) in row"
                :key="`square_${rowIndex}_${squareIndex}`"
                :class="`map-square_${square}`"
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
import { useGameBoardStore } from '~/stores/GameBoardStore';

const gameBoardStore = useGameBoardStore();
</script>

<style lang="scss">
.board {
    position: relative;
}

.square-row {
    display: flex;
}

.square {
    &.map-square_1 {
        background-color: #eee;
    }

    &.map-square_2 {
        background-color: #EC9009;
    }

    &.map-square_3 {
        background-color: #15E3DB;
    }
}
</style>
