<template>
    <div>
        <div
            v-for="(user, id) in roomStore.users"
            :key="id"
            class="user"
        >
            {{ user.username }}
            <div class="extra">
                <div class="role">
                    <template v-if="roomStore.owner === id">
                        {{ $t('room.userRole.owner') }}
                    </template>
                    <template v-else-if="roomStore.opponent === id">
                        {{ $t('room.userRole.opponent') }}
                    </template>
                    <template v-else>
                        {{ $t('room.userRole.spectator') }}
                    </template>
                </div>
                <div
                    v-if="(roomStore.owner === id || roomStore.opponent === id) && user.deck == null"
                    class="error"
                >
                    {{ $t('room.userError.deckNotChosen') }}
                </div>
            </div>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { useRoomStore } from '~/stores/RoomStore';

const roomStore = useRoomStore();
</script>

<style lang="scss" scoped>
.user {
    border: 2px solid $accent;
    background-color: $accent-a20;
    padding: 12px 18px;
    font-size: 1.5em;
    font-weight: 600;
    margin-bottom: 12px;
    display: flex;
    justify-content: space-between;
    align-items: center;

    > .extra {
        text-align: right;
        font-size: 0.85em;
        font-weight: 400;
    }

    .role {
        opacity: 0.75;
    }

    .error {
        margin-top: 2px;
        background-color: $error-red-a50;
        padding: 0 10px;
        border-radius: 9999px;
    }
}
</style>
