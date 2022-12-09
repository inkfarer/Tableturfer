<template>
    <Overlay
        v-model="overlayOpen"
        :closable="false"
        hide-background
        class="critical-error-overlay"
    >
        <p>{{ $t('game.error.communicationError') }}</p>
        <TtButton
            inline
            @click="returnHome"
        >
            {{ $t('game.error.backHome') }}
        </TtButton>
    </Overlay>
</template>

<script lang="ts" setup>
import { ref, watch } from '#imports';
import { useRoomStore } from '~/stores/RoomStore';
import { navigateTo } from '#app';

const overlayOpen = ref(false);

const roomStore = useRoomStore();
watch(() => roomStore.roomCode, newValue => {
    if (newValue == null) {
        overlayOpen.value = true;
    }
}, { immediate: true });

async function returnHome() {
    await navigateTo('/');
}
</script>

<style lang="scss">
.critical-error-overlay {
    .overlay {
        min-width: 450px !important;
        text-align: center;
    }
}
</style>
