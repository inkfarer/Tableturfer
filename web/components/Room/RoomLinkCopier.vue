<template>
    <div
        class="room-link-copier"
        :class="{ 'success': copyComplete }"
        :title="$t('roomLinkCopier.title')"
        @click="copyRoomCode"
    >
        <Icon
            v-if="clipboard.isSupported"
            :name="copyComplete ? 'fa6-solid:check' : 'fa6-solid:copy'"
        />
    </div>
</template>

<script lang="ts" setup>
import { useRoomStore } from '~/stores/RoomStore';
import { ref, useClipboard } from '#imports';

const roomStore = useRoomStore();
const clipboard = useClipboard();

const copyComplete = ref(false);
const resetCompletionTimeout = ref<number | undefined>();

function copyRoomCode() {
    if (roomStore.roomCode != null) {
        clipboard.copy(`${window.location.origin}/room/${roomStore.roomCode}`);

        copyComplete.value = true;
        window.clearTimeout(resetCompletionTimeout.value);
        resetCompletionTimeout.value = window.setTimeout(() => {
            copyComplete.value = false;
        }, 5000);
    }
}
</script>

<style lang="scss" scoped>
.room-link-copier {
    cursor: pointer;
    font-size: 0.4em;
    opacity: 0.5;
    transition: opacity $default-transition-duration;
    padding: 8px;

    &:hover {
        opacity: 0.75;
    }

    &:active {
        opacity: 0.9;
    }

    &.success {
        color: $success-green;
    }
}
</style>
