<template>
    <div class="game-stage-layout">
        <div class="side-section">
            <GameCardSelector class="top-margin" />
            <button @click="leaveRoom">LEAVE</button>
        </div>
        <GameBoard class="game-board" />
        <div class="side-section">
            <GameDebugInfo />
            <GameCardActionKeys class="top-margin" />
        </div>
    </div>
</template>

<script lang="ts" setup>
import { navigateTo } from '#app';
import { useNuxtApp } from '#imports';

const { $socket } = useNuxtApp();

async function leaveRoom() {
    $socket.disconnect();
    await navigateTo('/');
}
</script>

<style lang="scss">
.game-stage-layout {
    max-width: 1000px;
    display: grid;
    grid-template-columns: 1fr auto 1fr;
    gap: 10px;
}

.game-board {
    z-index: -1;
}

.side-section {
    z-index: 2;
}

.top-margin {
    margin-top: 10px;
}
</style>
