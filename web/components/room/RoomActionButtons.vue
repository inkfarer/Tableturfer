<template>
    <RoomDeckSelector
        v-if="roomStore.isRoomOwner || roomStore.isOpponent"
        ref="deckSelector"
    />
    <RoomMapSelector
        v-if="roomStore.isRoomOwner"
        ref="mapSelector"
    />

    <div v-bind="$attrs">
        <TtButton
            v-if="roomStore.isRoomOwner"
            class="mb-1x"
            @click="$refs.mapSelector.open()"
        >
            {{ $t('room.setMap') }}
        </TtButton>
        <TtButton
            v-if="roomStore.isOpponent || roomStore.isRoomOwner"
            class="mb-1x"
            @click="$refs.deckSelector.open()"
        >
            {{ $t('room.setDeck') }}
        </TtButton>
        <TtButton
            class="mb-1x"
            @click="leaveRoom"
        >
            {{ $t('room.leave') }}
        </TtButton>
        <TtButton
            v-if="roomStore.isRoomOwner"
            :disabled="!canStartGame"
            @click="startGame"
        >
            {{ $t('room.startGame') }}
        </TtButton>
    </div>
</template>

<script lang="ts" setup>
import { useRoomStore } from '~/stores/RoomStore';
import { useDeckStore } from '~/stores/DeckStore';
import { navigateTo } from '#app';
import { computed, useNuxtApp } from '#imports';

const { $socket } = useNuxtApp();
const roomStore = useRoomStore();
const deckStore = useDeckStore();

async function leaveRoom() {
    $socket.disconnect();
    await navigateTo('/');
}

const canStartGame = computed(() => {
    return roomStore.opponent != null && deckStore.deck != null && deckStore.opponentDeck != null;
});

function startGame() {
    $socket.send('StartGame');
}
</script>
