<template>
    <div class="game-wrapper">
        <Sandbox />
    </div>
</template>

<script lang="ts" setup>
import { definePageMeta } from '#imports';
import { useGameBoardStore } from '~/stores/GameBoardStore';

definePageMeta({
    layout: 'no-pull-to-refresh'
});

const gameBoardStore = useGameBoardStore();

gameBoardStore.setBoardByName('Square');
</script>

<style lang="scss">
body {
    overscroll-behavior: contain;
    -webkit-overflow-scrolling: auto;
}
</style>
