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
                    <RoomInfoRows />
                    <RoomActionButtons class="mt-1x" />
                </div>
                <div>
                    <RoomUserList />
                </div>
            </div>
        </div>
    </NuxtLayout>
</template>

<script lang="ts" setup>
import { definePageMeta, onMounted, ref, useNuxtApp, useRoute, useState, watch } from '#imports';
import { navigateTo } from '#app';
import { useRoomStore } from '~/stores/RoomStore';
import { useDeckListStore } from '~/stores/DeckListStore';
import { isBlank } from '~/helpers/StringHelper';

// Override the default page key so changing the room code in the URL (For example, /room/new -> /room/ASDF) doesn't make this component reload
definePageMeta({
    key: 'room',
    layout: false
});

const roomStore = useRoomStore();
const deckListStore = useDeckListStore();

const { $socket } = useNuxtApp();
const isLoading = ref(true);
const isError = ref(false);

onMounted(() => {
    // todo: it might be easier to initiate the ws connection (therefore getting a room code) __before__ directing users to this page
    watch(() => useRoute().params.code as string, async (newValue) => {
        if (newValue.toUpperCase() === roomStore.roomCode && $socket.isOpen()) {
            isLoading.value = false;
            return;
        }

        await connect(newValue);
    }, { immediate: true });

    watch(() => roomStore.started, async (newValue) => {
        if (newValue) {
            await navigateTo('/play');
        }
    }, { immediate: true });

    deckListStore.load();
});

async function connect(roomCode: string) {
    $socket.disconnect();
    isLoading.value = true;
    isError.value = false;

    try {
        roomStore.roomCode = await $socket.connect(roomCode.toLowerCase() === 'new' ? undefined : roomCode);
        if (roomStore.roomCode !== roomCode.toUpperCase()) {
            await navigateTo(`/room/${roomStore.roomCode}`, { replace: true });
        }
    } catch (e) {
        console.error(e);
        isError.value = true;
    } finally {
        isLoading.value = false;
    }
}

async function leaveRoom() {
    $socket.disconnect();
    await navigateTo('/');
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
