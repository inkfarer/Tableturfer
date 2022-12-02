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
            used cards: {{ deckStore.usedCards }}
        </div>
    </div>
</template>

<script lang="ts" setup>
import { useActiveCardStore } from '~/stores/ActiveCardStore';
import { useGameBoardStore } from '~/stores/GameBoardStore';
import { useRoomStore } from '~/stores/RoomStore';
import { useDeckStore } from '~/stores/DeckStore';

const activeCardStore = useActiveCardStore();
const gameBoardStore = useGameBoardStore();
const roomStore = useRoomStore();
const deckStore = useDeckStore();
</script>
