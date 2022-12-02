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
            <button @click="activeCardStore.position = { x: overrideX, y: overrideY }">Override Position</button>
        </div>

        <label>
            special attack!
            <input
                v-model="activeCardStore.special"
                type="checkbox"
            >
        </label>

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
import { ref, useNuxtApp } from '#imports';
import { useRoomStore } from '~/stores/RoomStore';

const activeCardStore = useActiveCardStore();
const roomStore = useRoomStore();
const { $socket } = useNuxtApp();

function placeCard() {
    if (activeCardStore.activeCard == null || roomStore.playerTeam == null) {
        return;
    }

    if (!activeCardStore.pass) {
        $socket.send('ProposeMove', {
            type: 'PlaceCard',
            cardName: activeCardStore.activeCard.name,
            position: activeCardStore.position,
            rotation: activeCardStore.rotation,
            special: activeCardStore.special
        });
    } else {
        $socket.send('ProposeMove', {
            type: 'Pass',
            cardName: activeCardStore.activeCard.name
        });
    }
    activeCardStore.setActiveCard(null);
    activeCardStore.special = false;
    activeCardStore.pass = false;
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
