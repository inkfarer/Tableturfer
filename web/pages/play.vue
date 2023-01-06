<template>
    <div class="game-wrapper">
        <Game />
        <GameCriticalErrorOverlay />
    </div>
</template>

<script lang="ts" setup>
import { definePageMeta } from '#imports';

definePageMeta({
    layout: false
});
</script>

<style lang="scss">
body {
    overscroll-behavior: contain;
}

.game-wrapper {
    color: white;
}
</style>
