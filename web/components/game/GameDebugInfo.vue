<template>
    <div>
        <div>
            active card: {{ activeCardStore.activeCard?.name ?? 'None' }}
        </div>
        <div>
            squares: {{ activeCardStore.activeCard?.squares ?? 'None' }}
        </div>
        <div>
            pos: {{ activeCardStore.position }}
        </div>
        <div>
            rot: {{ activeCardStore.rotation }}
        </div>
        <div>
            team: {{ roomStore.playerTeam }}
        </div>
        <div>
            board: {{ gameBoardStore.name }}
        </div>
    </div>
</template>

<script lang="ts" setup>
import { useActiveCardStore } from '~/stores/ActiveCardStore';
import { useGameBoardStore } from '~/stores/GameBoardStore';
import { useRoomStore } from '~/stores/RoomStore';

const activeCardStore = useActiveCardStore();
const gameBoardStore = useGameBoardStore();
const roomStore = useRoomStore();
</script>
