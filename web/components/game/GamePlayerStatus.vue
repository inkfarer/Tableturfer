<template>
    <div class="player-status">
        <div>
            Player 1
            <GamePlayerSpecialCount :team="PlayerTeam.ALPHA" />
        </div>
        <div class="turn-counter">
            <div class="label">{{ $t('game.turnsRemaining') }}</div>
            <div class="turn-count-number">{{ roomStore.remainingTurns }}</div>
        </div>
        <div>
            Player 2
            <GamePlayerSpecialCount :team="PlayerTeam.BRAVO" />
        </div>
    </div>
</template>

<script lang="ts" setup>
import { PlayerTeam } from '~/types/PlayerTeam';
import { useRoomStore } from '~/stores/RoomStore';

const roomStore = useRoomStore();
</script>

<style lang="scss" scoped>
.player-status {
    display: flex;
    justify-content: space-between;
    align-items: flex-end;
    font-size: 1.1em;
    font-weight: 500;

    > *:last-child {
        text-align: right;
    }

    .special-count {
        margin-top: 2px;
    }

    .turn-counter {
        text-align: center;

        .label {
            font-size: 0.9em;
            font-weight: 400;
        }

        .turn-count-number {
            font-size: 2em;
            font-weight: 800;
        }
    }
}
</style>
