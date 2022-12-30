<template>
    <div class="board-base">
        <div
            v-for="(row, rowIndex) in gameBoardStore.baseBoard"
            :key="`row_${rowIndex}`"
            class="square-row"
        >
            <div
                v-for="(square, squareIndex) in row"
                :key="`square_${rowIndex}_${squareIndex}`"
                :class="`map-square_${square === MapSquareType.DISABLED ? MapSquareType.DISABLED : MapSquareType.EMPTY}`"
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
import { useGameBoardStore } from '~/stores/GameBoardStore';
import Constants from '~/data/Constants';
import { MapSquareType } from '~/types/MapSquareType';

const gameBoardStore = useGameBoardStore();
</script>

<style lang="scss" scoped>
.board-base {
    position: absolute;
    z-index: -1;
}

.square {
    box-sizing: border-box;

    &.map-square_1 {
        background-color: #171717;
        border: 1px solid #393939;
    }
}
</style>
