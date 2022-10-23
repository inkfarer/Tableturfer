<template>
    <div class="stage-selector">
        <div>
            active stage: {{ gameBoardStore.name }}
        </div>
        <button
            v-for="map in Maps"
            :key="`map_${map.name}`"
            @click="gameBoardStore.setBoard(map)"
        >
            {{ map.name }}
        </button>
    </div>
</template>

<script lang="ts" setup>
import * as Maps from '~/data/maps';
import { useGameBoardStore } from '~/stores/GameBoardStore';

const gameBoardStore = useGameBoardStore();
</script>

<style lang="scss">
.stage-selector {
    button {
        display: block;

        &:not(:first-child) {
            margin-top: 4px;
        }
    }
}
</style>
