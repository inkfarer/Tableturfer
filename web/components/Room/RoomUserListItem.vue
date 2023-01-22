<template>
    <div class="user">
        {{ user.username }}
        <div class="extra">
            <div class="role">
                {{ roleText }}
            </div>
            <div
                v-if="(roomStore.owner === userId || roomStore.opponent === userId) && user.deck == null"
                class="error"
            >
                {{ $t('room.userError.deckNotChosen') }}
            </div>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { SocketUser } from '~/types/socket/SocketEvent';
import { useRoomStore } from '~/stores/RoomStore';
import { computed, useI18n } from '#imports';

const props = defineProps<{
    user: SocketUser,
    userId: string
}>();


const roomStore = useRoomStore();
const i18n = useI18n();


const roleText = computed(() => {
    if (props.userId === roomStore.owner) {
        return i18n.t('room.userRole.owner');
    } else if (props.userId === roomStore.opponent) {
        return i18n.t('room.userRole.opponent');
    } else {
        return i18n.t('room.userRole.spectator');
    }
});
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

@include media-breakpoint-down(sm) {
    .user {
        flex-direction: column;

        > .extra {
            margin-top: 6px;
            text-align: center;
            display: flex;
            align-items: center;
            justify-content: center;
            flex-wrap: wrap;

            > *:not(:last-child) {
                margin-right: 4px;
            }
        }
    }
}
</style>
