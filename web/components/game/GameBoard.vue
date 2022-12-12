<template>
    <div
        class="board"
        :class="{
            'special-attack': activeCardStore.special
        }"
    >
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
import { useCurrentMoveStore } from '~/stores/CurrentMoveStore';

const activeCardStore = useCurrentMoveStore();
const gameBoardStore = useGameBoardStore();
</script>

<style lang="scss">
.board {
    position: relative;
    height: max-content;

    &.special-attack {
        .map-square_5 {
            opacity: 0.1;
        }

        .map-square_4 {
            opacity: 0.3;
        }
    }
}

.square-row {
    display: flex;
}

.square {
    box-sizing: border-box;

    &.map-square_1 {
        background-color: #171717;
        border: 1px solid #393939;
    }

    &.map-square_2 {
        background-color: #EC9009;
    }

    &.map-square_3 {
        background-color: #15E3DB;
    }

    &.map-square_4 {
        background-color: #E9FF0F;
    }

    &.map-square_5 {
        background-color: #4B50F3;
    }

    &.map-square_6 {
        background-color: #aaa;
    }

    &.map-square_8 {
        background-color: #FFBC5A;
    }

    &.map-square_9 {
        background-color: #A4FFFB;
    }
}
</style>
