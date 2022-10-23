<template>
    <div class="card-movement-keys">
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
    </div>
</template>

<script lang="ts" setup>
import { useActiveCardStore } from '~/stores/ActiveCardStore';

const activeCardStore = useActiveCardStore();
</script>

<style lang="scss">
.card-movement-keys {
    display: flex;
    flex-direction: column;
    align-items: center;
}
</style>
