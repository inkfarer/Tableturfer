<template>
    <div class="room-joiner-wrapper">
        <div class="mb-2x username-input">
            <PlayerNameInput
                @update:is-valid="nameIsValid = $event"
            />
        </div>
        <div
            class="room-joiner"
            :class="{ 'disabled': roomJoiningDisabled }"
        >
            <div>
                <TtButton
                    :disabled="roomJoiningDisabled"
                    @click="createRoom"
                >
                    {{ $t('roomJoiner.newRoom') }}
                </TtButton>
            </div>
            <div class="join-existing">
                <RoomJoinerCodeInput v-model="roomCode" />
                <TtButton
                    class="mt-2x"
                    :disabled="roomJoiningDisabled"
                    @click="joinRoom"
                >
                    {{ $t('roomJoiner.joinRoom') }}
                </TtButton>
            </div>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { navigateTo, ref, computed } from '#imports';

const roomCode = ref('');
const nameIsValid = ref(false);

async function createRoom() {
    await navigateTo('/room/new');
}

async function joinRoom() {
    await navigateTo(`/room/${roomCode.value}`);
}

const roomJoiningDisabled = computed(() => !nameIsValid.value);
</script>

<style lang="scss" scoped>
.room-joiner-wrapper {
    > * {
        border-radius: 16px;
        padding: 15px 15px;
        background-color: $accent-a20;
        backdrop-filter: blur(5px);
        border: 2px solid $accent;
    }
}

.username-input {
    display: inline-flex;
    justify-content: center;
    text-align: initial;
}

.room-joiner {
    display: flex;
    transition: opacity 250ms;

    > * {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        text-align: center;
        width: 100%;
        padding: 10px 0;
        border-width: 0;
        border-color: $accent;
        border-style: solid;

        &:not(:last-child) {
            border-right-width: 2px;
        }
    }

    &.disabled {
        opacity: 0.5;
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
