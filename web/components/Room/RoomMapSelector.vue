<template>
    <Overlay v-model="isOpen">
        <div class="stage-selector">
            <TtButton
                v-for="[name, map] in GameMapMap"
                :key="`map_${name}`"
                @click="setMap(name)"
            >
                {{ $t(`game.map.${map.name}`) }}
            </TtButton>
        </div>
    </Overlay>
</template>

<script lang="ts" setup>
import { GameMapMap } from '~/helpers/Maps';
import { ref, useNuxtApp } from '#imports';

const isOpen = ref(false);

function open() {
    isOpen.value = true;
}

const { $socket } = useNuxtApp();

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
