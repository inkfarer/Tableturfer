<template>
    <div class="room-joiner">
        <div>
            <TtButton @click="createRoom">
                {{ $t('roomJoiner.newRoom') }}
            </TtButton>
        </div>
        <div class="join-existing">
            <RoomJoinerCodeInput v-model="roomCode" />
            <TtButton @click="joinRoom">
                {{ $t('roomJoiner.joinRoom') }}
            </TtButton>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { navigateTo, ref } from '#imports';

const roomCode = ref('');

async function createRoom() {
    await navigateTo('/room/new');
}

async function joinRoom() {
    await navigateTo(`/room/${roomCode.value}`);
}
</script>

<style lang="scss" scoped>
.room-joiner {
    border-radius: 16px;
    border: 2px solid $accent;
    padding: 15px 15px;
    background-color: $accent-a20;
    backdrop-filter: blur(5px);
    display: flex;

    > * {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        text-align: center;
        width: 100%;
        padding: 20px 0;
        border-width: 0;
        border-color: $accent;
        border-style: solid;

        &:not(:last-child) {
            border-right-width: 2px;
        }
    }

    .join-existing > .button {
        margin-top: 16px;
    }
}

@include media-breakpoint-down(md) {
    .room-joiner {
        flex-direction: column;

        > *:not(:last-child) {
            border-right-width: 0;
            border-bottom-width: 2px;
        }
    }
}
</style>
