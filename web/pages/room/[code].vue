<template>
    <div>
        <div v-if="isLoading">
            loading...
        </div>
        <div v-else-if="isError">
            there has been an error
            <br>
            <button @click="leaveRoom">back home</button>
        </div>
        <div v-else>
            joined room {{ roomStore.roomCode }} with users {{ roomStore.users }}
            <div v-if="roomStore.isRoomOwner">
                you own this room!
            </div>
            <br>
            <button @click="leaveRoom">cool! can i go back home now</button>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { definePageMeta, onMounted, ref, useNuxtApp, useRoute, watch } from '#imports';
import { navigateTo } from '#app';
import { useRoomStore } from '~/stores/RoomStore';

// Override the default page key so changing the room code in the URL (For example, /room/new -> /room/ASDF) doesn't make this component reload
definePageMeta({
    key: 'room'
});

const roomStore = useRoomStore();
const connectedRoomCode = ref<string | null>(null);
const { $socket } = useNuxtApp();
const isLoading = ref(true);
const isError = ref(false);

onMounted(() => {
    watch(() => useRoute().params.code as string, async (newValue) => {
        if (newValue.toUpperCase() === connectedRoomCode.value) {
            return;
        }

        connectedRoomCode.value = null;
        isLoading.value = true;
        isError.value = false;

        try {
            connectedRoomCode.value = await $socket.connect(newValue.toLowerCase() === 'new' ? undefined : newValue);
            if (connectedRoomCode.value !== newValue.toUpperCase()) {
                await navigateTo(`/room/${connectedRoomCode.value}`, { replace: true });
            }
        } catch (e) {
            console.error(e);
            isError.value = true;
        } finally {
            isLoading.value = false;
        }
    }, { immediate: true });
});

async function leaveRoom() {
    $socket.disconnect();
    await navigateTo('/');
}
</script>
