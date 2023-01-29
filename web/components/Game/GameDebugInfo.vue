<template>
    <div class="debug-info">
        <div>
            active card: {{ activeCardStore.activeCard?.name ?? 'None' }}
        </div>
        <div>
            pos: {{ activeCardStore.position }}
        </div>
        <div>
            origin: {{ activeCardStore.activeCard?.origin ?? 'None' }}
        </div>
        <div>
            rot: {{ activeCardStore.rotation }}
        </div>
        <div>
            team: {{ roomStore.playerTeam }}
        </div>
        <div>
            turns remaining: {{ roomStore.remainingTurns }}
        </div>
        <div>
            board: {{ gameBoardStore.name }}
        </div>
        <div>
            special: {{ gameBoardStore.specialPointCount }}
        </div>
        <div>
            used sp: {{ gameBoardStore.usedSpecialPoints }}
        </div>
        <div>
            move locked? {{ activeCardStore.locked }}
        </div>
    </div>
</template>

<script lang="ts" setup>
import { useCurrentMoveStore } from '~/stores/CurrentMoveStore';
import { useGameBoardStore } from '~/stores/GameBoardStore';
import { useRoomStore } from '~/stores/RoomStore';

const activeCardStore = useCurrentMoveStore();
const gameBoardStore = useGameBoardStore();
const roomStore = useRoomStore();
</script>

<style lang="scss" scoped>
.debug-info {
    position: absolute;
    top: 0;
    left: 0;
    background-color: rgba(0, 0, 0, 0.5);
    opacity: 0.9;
    pointer-events: none;
    z-index: 999;
}
</style>
