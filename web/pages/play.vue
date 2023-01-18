<template>
    <div class="game-wrapper">
        <Game />
        <GameCriticalErrorOverlay />
    </div>
</template>

<script lang="ts" setup>
import { definePageMeta } from '#imports';

definePageMeta({
    layout: 'no-pull-to-refresh'
});
</script>

<style lang="scss">
.game-wrapper {
    color: white;
}
</style>
