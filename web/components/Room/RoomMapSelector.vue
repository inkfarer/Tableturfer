<template>
    <Overlay v-model="isOpen">
        <div class="stage-selector">
            <TtButton
                v-for="name in mapNames"
                :key="`map_${name}`"
                @click="setMap(name)"
            >
                {{ $t(`game.map.${name}`) }}
            </TtButton>
        </div>
    </Overlay>
</template>

<script lang="ts" setup>
import { GameMapMap, RANDOM_MAP_NAME } from '~/helpers/Maps';
import { ref, useNuxtApp } from '#imports';

const isOpen = ref(false);

function open() {
    isOpen.value = true;
}

const { $socket } = useNuxtApp();

const mapNames = [
    RANDOM_MAP_NAME,
    ...GameMapMap.keys()
];

function setMap(name: string) {
    $socket.send('SetMap', name);
    isOpen.value = false;
}

defineExpose({
    open
});
</script>

<style lang="scss" scoped>
.stage-selector {
    .button {
        &:not(:last-child) {
            margin-bottom: 8px;
        }
    }
}
</style>
