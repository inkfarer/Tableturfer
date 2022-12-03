<template>
    <div class="stage-selector">
        <button
            v-for="[name, map] in GameMapMap"
            :key="`map_${name}`"
            @click="setMap(name)"
        >
            {{ map.name }}
        </button>
    </div>
</template>

<script lang="ts" setup>
import { GameMapMap } from '~/helpers/Maps';
import { useNuxtApp } from '#imports';

const { $socket } = useNuxtApp();

function setMap(name: string) {
    $socket.send('SetMap', name);
}
</script>

<style lang="scss" scoped>
.stage-selector {
    button {
        &:not(:last-child) {
            margin-right: 4px;
        }
    }
}
</style>
