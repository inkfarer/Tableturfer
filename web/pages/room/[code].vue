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
                    <h1>
                        {{ roomStore.roomCode }}
                        <RoomLinkCopier class="room-link-copier" />
                    </h1>
                </template>
            </div>
        </template>
        <div class="width-cap room">
            <RoomMissingUsernameOverlay
                :visible="showUsernameOverlay"
                @connect="connectAfterMissingUsername"
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
import {
    computed,
    definePageMeta, initUsernameAfterLoad,
    onMounted,
    preloadRouteComponents,
    ref,
    useHead,
    useI18n,
    useNuxtApp,
    useRoute,
    watch
} from '#imports';
import { navigateTo } from '#app';
import { useRoomStore } from '~/stores/RoomStore';
import { useDeckListStore } from '~/stores/DeckListStore';
import { isBlank } from '~/helpers/StringHelper';
import RoomMissingUsernameOverlay from '~/components/Room/RoomMissingUsernameOverlay.vue';
import { useUsername } from '~/utils/UseUsername';

// Override the default page key so changing the room code in the URL (For example, /room/new -> /room/ASDF) doesn't make this component reload
definePageMeta({
    key: 'room',
    layout: false
});

preloadRouteComponents('/play');

const roomStore = useRoomStore();
const deckListStore = useDeckListStore();

const username = useUsername();
const showUsernameOverlay = ref(false);

const i18n = useI18n();
const route = useRoute();
const { $socket } = useNuxtApp();
const isLoading = ref(true);
const isError = ref(false);

useHead({
    title: computed(() => {
        if (isError.value) {
            return null;
        } else if (isLoading.value) {
            return i18n.t('room.title.loading');
        } else {
            return i18n.t('room.title.joinedRoomCode', { code: roomStore.roomCode });
        }
    })
});

onMounted(() => {
    initUsernameAfterLoad();
    showUsernameOverlay.value = isBlank(username.value);
    // todo: it might be easier to initiate the ws connection (therefore getting a room code) __before__ directing users to this page
    watch(() => route.params.code as string, async (newValue) => {
        if (isBlank(username.value) || (newValue.toUpperCase() === roomStore.roomCode && $socket.isOpen())) {
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
        roomStore.roomCode = await $socket.connect(roomCode.toLowerCase() === 'new' ? undefined : roomCode, username.value ?? '');
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

async function connectAfterMissingUsername() {
    await connect(route.params.code as string);
    showUsernameOverlay.value = false;
}

async function leaveRoom() {
    $socket.disconnect();
    await navigateTo('/');
}
</script>

<style lang="scss" scoped>
@include media-breakpoint-down(md) {
    .room-layout {
        grid-template-columns: 1fr !important;
    }
}

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
        position: relative;

        .room-link-copier {
            display: inline-block;
            position: absolute;
            bottom: 50%;
            margin-left: 5px;
            transform: translateY(50%);
        }
    }
}

.room {
    margin-top: 20px;
    margin-bottom: 20px;

    .room-layout {
        display: grid;
        gap: 16px;
        grid-template-columns: 2fr 3fr;
    }
}
</style>
