<template>
    <div class="card-movement-keys">
        <div style="margin-bottom: 10px;">
            <input
                v-model="overrideX"
                type="number"
            >
            <input
                v-model="overrideY"
                type="number"
            >
            <button @click="activeCardStore.internalPosition = { x: overrideX, y: overrideY }">Override Position</button>
        </div>

        <button @click="activeCardStore.moveUp()">Up</button>
        <div>
            <button @click="activeCardStore.moveLeft()">Left</button>
            <button @click="activeCardStore.moveRight()">Right</button>
        </div>
        <button @click="activeCardStore.moveDown()">Down</button>
        <div style="margin-top: 10px;">
            <button @click="activeCardStore.previousRotationStep()">Spin Left</button>
            <button @click="activeCardStore.nextRotationStep()">Spin Right</button>
        </div>
        <div style="margin-top: 10px;">
            <button @click="placeCard">Place</button>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { useActiveCardStore } from '~/stores/ActiveCardStore';
import { useGameBoardStore } from '~/stores/GameBoardStore';
import { ref } from '#imports';

const activeCardStore = useActiveCardStore();
const gameBoardStore = useGameBoardStore();

function placeCard() {
    if (activeCardStore.activeCard == null) {
        return;
    }

    gameBoardStore.placeCard(activeCardStore.position, activeCardStore.activeCard.squares);
}

const overrideX = ref(0);
const overrideY = ref(0);
</script>

<style lang="scss">
.card-movement-keys {
    display: flex;
    flex-direction: column;
    align-items: center;
}
</style>
