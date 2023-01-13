<template>
    <RoomDeckSelector
        v-if="roomStore.isRoomOwner || roomStore.isOpponent"
        ref="deckSelector"
    />
    <RoomMapSelector
        v-if="roomStore.isRoomOwner"
        ref="mapSelector"
    />
    <GameGuideOverlay ref="guideOverlay" />
    <UserSettingsOverlay ref="userSettingsOverlay" />

    <div
        v-bind="$attrs"
        class="room-action-buttons"
    >
        <TtButton
            v-if="roomStore.isRoomOwner"
            @click="
                // @ts-ignore
                $refs.mapSelector.open()
            "
        >
            {{ $t('room.setMap') }}
        </TtButton>
        <TtButton
            v-if="roomStore.isOpponent || roomStore.isRoomOwner"
            @click="
                // @ts-ignore
                $refs.deckSelector.open()
            "
        >
            {{ $t('room.setDeck') }}
        </TtButton>
        <TtButton
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
        <div class="secondary-options">
            <TtButton
                theme="secondary"
                @click="
                    // @ts-ignore
                    $refs.guideOverlay.open()
                "
            >
                <Icon name="fa6-solid:question" /> {{ $t('room.howToPlay') }}
            </TtButton>
            <TtButton
                theme="secondary"
                @click="
                    // @ts-ignore
                    $refs.userSettingsOverlay.open()
                "
            >
                <Icon name="fa6-solid:gear" /> {{ $t('room.userSettings') }}
            </TtButton>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { useRoomStore } from '~/stores/RoomStore';
import { useDeckStore } from '~/stores/DeckStore';
import { navigateTo } from '#app';
import { computed, useNuxtApp } from '#imports';
import GameGuideOverlay from '~/components/game/GameGuideOverlay.vue';
import UserSettingsOverlay from '~/components/UserSettingsOverlay.vue';

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

<style lang="scss" scoped>
.room-action-buttons {
    display: grid;
    gap: 8px;
}

.secondary-options {
    border-top: 2px solid $accent;
    margin-top: 8px;
    padding-top: 8px;
    display: flex;
    justify-content: center;
}

@include media-breakpoint-only(sm) {
    .room-action-buttons {
        grid-template-columns: repeat(2, 1fr);
    }

    .secondary-options {
        grid-column: 2 span;
    }
}
</style>
