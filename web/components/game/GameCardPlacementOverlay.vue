<template>
    <div
        class="card-placement-overlay"
        :class="{ placeable, [`team_${roomStore.playerTeam}`]: true, hidden: activeCardStore.pass }"
        :style="{
            transform: `translate(${Constants.BOARD_SQUARE_SIZE_PX * activeCardStore.position.x}px, ${Constants.BOARD_SQUARE_SIZE_PX * activeCardStore.position.y}px)`
        }"
    >
        <ClientOnly>
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
        </ClientOnly>
    </div>
</template>

<script lang="ts" setup>
import Constants from '~/data/Constants';
import { useCurrentMoveStore } from '~/stores/CurrentMoveStore';
import { useGameBoardStore } from '~/stores/GameBoardStore';
import { computed } from '#imports';
import { useRoomStore } from '~/stores/RoomStore';

const activeCardStore = useCurrentMoveStore();
const gameBoardStore = useGameBoardStore();
const roomStore = useRoomStore();

const placeable = computed(() => {
    if (activeCardStore.activeCard == null) {
        return false;
    }

    return gameBoardStore.isPlaceable(activeCardStore.position, activeCardStore.activeCard.squares);
});

</script>

<style lang="scss">
.card-placement-overlay {
    position: absolute;

    &:not(.placeable) {
        filter: grayscale(100%);
    }

    &.hidden {
        visibility: hidden;
    }

    &.team_Alpha .square {
        &.card-square_1 {
            background-color: rgba(236, 144, 9, 0.5);
        }

        &.card-square_2 {
            background-color: rgba(236, 144, 9, 0.8);
        }
    }

    &.team_Bravo .square {
        &.card-square_1 {
            background-color: rgba(75, 80, 243, 0.2);
        }

        &.card-square_2 {
            background-color: rgba(21, 227, 219, 0.5);
        }
    }
}
</style>
