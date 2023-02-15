<template>
    <DataRow :label="$t('room.mapName')">
        {{ $t(`game.map.${gameBoardStore.isRandomBoard ? RANDOM_MAP_NAME : gameBoardStore.name}`) }}
    </DataRow>
    <template v-if="roomStore.isOpponent || roomStore.isRoomOwner">
        <DataRow :label="$t('room.deckName')">
            {{ formatMissingValue(deckStore.deck?.name) }}
        </DataRow>
    </template>
</template>

<script lang="ts" setup>
import { formatMissingValue } from '#imports';
import { useGameBoardStore } from '~/stores/GameBoardStore';
import { useRoomStore } from '~/stores/RoomStore';
import { useDeckStore } from '~/stores/DeckStore';
import { RANDOM_MAP_NAME } from '~/helpers/Maps';

const gameBoardStore = useGameBoardStore();
const roomStore = useRoomStore();
const deckStore = useDeckStore();
</script>
