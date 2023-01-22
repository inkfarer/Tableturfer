<template>
    <div>
        <div class="header">
            {{ $t('room.userListTitle') }}
        </div>
        <RoomUserListItem
            v-for="(user, id) in roomStore.users"
            :key="id"
            :user-id="id"
            :user="user"
        />
    </div>
</template>

<script lang="ts" setup>
import { useRoomStore } from '~/stores/RoomStore';
import RoomUserListItem from '~/components/Room/RoomUserListItem.vue';

const roomStore = useRoomStore();
</script>

<style lang="scss" scoped>
.header {
    font-size: 1.25em;
    font-weight: 600;
    border-bottom: 2px solid $accent;
    margin-bottom: 8px;
    padding-bottom: 4px;
}
</style>
