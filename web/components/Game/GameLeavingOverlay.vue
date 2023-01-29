<template>
    <Overlay v-model="isOpen">
        <p class="text-center">{{ $t('game.leavingOverlay.title') }}</p>
        <div class="options mt-2">
            <TtButton
                class="mr-1x"
                @click="onLeave"
            >
                {{ $t('game.leavingOverlay.confirm') }}
            </TtButton>
            <TtButton @click="onCancel">
                {{ $t('game.leavingOverlay.cancel') }}
            </TtButton>
        </div>
    </Overlay>
</template>

<script lang="ts" setup>
import { navigateTo, ref, useNuxtApp } from '#imports';

const isOpen = ref(false);
function open() {
    isOpen.value = true;
}

defineExpose({ open, isOpen: () => isOpen.value });

const { $socket } = useNuxtApp();

async function onLeave() {
    $socket.disconnect();
    await navigateTo('/');
}

function onCancel() {
    isOpen.value = false;
}
</script>

<style lang="scss" scoped>
.options {
    display: flex;

    > * {
        flex-grow: 1;
        flex-basis: 100%;
    }
}
</style>
