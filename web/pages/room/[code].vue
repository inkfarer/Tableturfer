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
            <template v-if="roomStore.isRoomOwner">
                <br>
                you own this room!
            </template>
            <template v-if="roomStore.isOpponent">
                <br>
                you are about to play!
            </template>
            <br>
            the map is "{{ gameBoardStore.name }}"
            <br>
            <RoomMapSelector v-if="roomStore.isRoomOwner" />
            <template v-if="roomStore.isRoomOwner">
                <button @click="startGame">
                    start the game!
                </button>
                <br>
            </template>
            <button @click="leaveRoom">cool! can i go back home now</button>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { definePageMeta, onMounted, ref, useNuxtApp, useRoute, watch } from '#imports';
import { navigateTo } from '#app';
import { useRoomStore } from '~/stores/RoomStore';
import { useGameBoardStore } from '~/stores/GameBoardStore';

// Override the default page key so changing the room code in the URL (For example, /room/new -> /room/ASDF) doesn't make this component reload
definePageMeta({
    key: 'room'
});

const gameBoardStore = useGameBoardStore();
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

    watch(() => roomStore.started, async (newValue) => {
        if (newValue) {
            await navigateTo('/play');
        }
    }, { immediate: true });
});

async function leaveRoom() {
    $socket.disconnect();
    await navigateTo('/');
}

function startGame() {
    $socket.send('StartGame');
}
</script>
