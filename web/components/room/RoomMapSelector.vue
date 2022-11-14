<template>
    <div class="stage-selector">
        <button
            v-for="map in Maps"
            :key="`map_${map.name}`"
            @click="setMap(map.name)"
        >
            {{ map.name }}
        </button>
    </div>
</template>

<script lang="ts" setup>
import * as Maps from '~/data/maps';
import { useNuxtApp } from '#imports';

const { $socket } = useNuxtApp();

function setMap(name: string) {
    $socket.send('SetMap', name);
}
</script>

<style lang="scss" scoped>
.stage-selector {
    button {
        &:not(:first-child) {
            margin-right: 4px;
        }
    }
}
</style>
