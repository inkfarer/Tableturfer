<template>
    <NuxtLayout name="default">
        <template #header>
            <div
                v-if="!isError"
                class="width-cap room-code-display"
            >
                <h1 v-if="isLoading">
                    {{ $t('room.loading') }}
                </h1>
                <template v-else>
                    <h4>{{ $t('room.beforeRoomCode') }}</h4>
                    <h1>{{ roomStore.roomCode }}</h1>
                </template>
            </div>
        </template>
        <div class="width-cap room">
            <RoomDeckSelector
                v-if="roomStore.isRoomOwner || roomStore.isOpponent"
                ref="deckSelector"
            />
            <RoomMapSelector
                v-if="roomStore.isRoomOwner"
                ref="mapSelector"
            />
            <div
                v-if="!isLoading && isError"
                class="text-center"
            >
                <p>{{ $t('room.errorOccurred') }}</p>
                <TtButton
                    inline
                    @click="leaveRoom"
                >
                    {{ $t('room.backHome') }}
                </TtButton>
            </div>
            <div
                v-else-if="!isLoading && !isError"
                class="room-layout"
            >
                <div>
                    <DataRow :label="$t('room.mapName')">
                        {{ $t(`game.map.${gameBoardStore.name}`) }}
                    </DataRow>
                    <template v-if="roomStore.isOpponent || roomStore.isRoomOwner">
                        <DataRow :label="$t('room.deckName')">
                            {{ formatMissingValue(deckStore.deck?.name) }}
                        </DataRow>
                    </template>
                    <TtButton
                        v-if="roomStore.isRoomOwner"
                        @click="updateMap"
                    >
                        {{ $t('room.setMap') }}
                    </TtButton>
                    <TtButton
                        v-if="roomStore.isOpponent || roomStore.isRoomOwner"
                        @click="updateDeck"
                    >
                        {{ $t('room.setDeck') }}
                    </TtButton>
                    <TtButton @click="leaveRoom">{{ $t('room.leave') }}</TtButton>
                    <TtButton
                        v-if="roomStore.isRoomOwner"
                        :disabled="!canStartGame"
                        @click="startGame"
                    >
                        {{ $t('room.startGame') }}
                    </TtButton>
                </div>
                <div>
                    <RoomUserList />
                </div>
            </div>
        </div>
    </NuxtLayout>
</template>

<script lang="ts" setup>
import { computed, definePageMeta, onMounted, ref, useNuxtApp, useRoute, watch } from '#imports';
import { navigateTo } from '#app';
import { useRoomStore } from '~/stores/RoomStore';
import { useGameBoardStore } from '~/stores/GameBoardStore';
import { useDeckStore } from '~/stores/DeckStore';
import { formatMissingValue } from '#imports';
import { RoomDeckSelector, RoomMapSelector } from '#components';
import { ComponentPublicInstance } from 'vue';
import { useDeckListStore } from '~/stores/DeckListStore';

// Override the default page key so changing the room code in the URL (For example, /room/new -> /room/ASDF) doesn't make this component reload
definePageMeta({
    key: 'room',
    layout: false
});

const deckStore = useDeckStore();
const gameBoardStore = useGameBoardStore();
const roomStore = useRoomStore();
const deckListStore = useDeckListStore();

const { $socket } = useNuxtApp();
const isLoading = ref(true);
const isError = ref(false);
const deckSelector = ref<ComponentPublicInstance<typeof RoomDeckSelector> | null>(null);
const mapSelector = ref<ComponentPublicInstance<typeof RoomMapSelector> | null>(null);

onMounted(() => {
    // todo: it might be easier to initiate the ws connection (therefore getting a room code) __before__ directing users to this page
    watch(() => useRoute().params.code as string, async (newValue) => {
        if (newValue.toUpperCase() === roomStore.roomCode && $socket.isOpen()) {
            isLoading.value = false;
            return;
        }

        $socket.disconnect();
        isLoading.value = true;
        isError.value = false;

        try {
            roomStore.roomCode = await $socket.connect(newValue.toLowerCase() === 'new' ? undefined : newValue);
            if (roomStore.roomCode !== newValue.toUpperCase()) {
                await navigateTo(`/room/${roomStore.roomCode}`, { replace: true });
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

    deckListStore.load();
});

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

function updateDeck() {
    deckSelector.value?.open();
}

function updateMap() {
    mapSelector.value?.open();
}
</script>

<style lang="scss" scoped>
.room-code-display {
    text-align: center;

    > h4 {
        font-weight: 500;
        margin-bottom: 0;
        font-size: 1.2em;
    }

    > h1 {
        margin-top: 0;
        margin-bottom: 20px;
        font-size: 4em;
    }
}

.room {
    margin-top: 20px;

    .room-layout {
        display: flex;

        > *:nth-child(1) {
            width: 35%;
            margin-right: 16px;

            .button {
                margin-top: 10px;
            }
        }

        > *:nth-child(2) {
            width: 65%;
        }
    }
}
</style>
