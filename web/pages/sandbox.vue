<template>
    <div class="game-wrapper">
        <Sandbox />
    </div>
</template>

<script lang="ts" setup>
import { definePageMeta } from '#imports';
import { useGameBoardStore } from '~/stores/GameBoardStore';

definePageMeta({
    layout: false
});

const gameBoardStore = useGameBoardStore();

gameBoardStore.setBoardByName('Square');
</script>

<style lang="scss">
body {
    overscroll-behavior: contain;
}
</style>
