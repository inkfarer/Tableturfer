<template>
    <div>
        <div>
            active card: {{ activeCardStore.activeCard?.name ?? 'None' }}
        </div>
        <div>
            pos: {{ activeCardStore.position }}
        </div>
        <div>
            rot: {{ activeCardStore.rotation }}
        </div>
        <div>
            team: {{ gameStateStore.playerTeam }}
        </div>
        <div>
            board: {{ gameBoardStore.name }}
        </div>
    </div>
</template>

<script lang="ts" setup>
import { useActiveCardStore } from '~/stores/ActiveCardStore';
import { useGameStateStore } from '~/stores/GameStateStore';
import { useGameBoardStore } from '~/stores/GameBoardStore';

const activeCardStore = useActiveCardStore();
const gameStateStore = useGameStateStore();
const gameBoardStore = useGameBoardStore();
</script>
