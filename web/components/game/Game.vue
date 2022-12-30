<template>
    <div
        class="game-stage-layout"
        :class="{ passing: activeCardStore.pass }"
    >
        <div class="side-section card-selector-section">
            <GameCardSelector class="card-selector" />
        </div>
        <div class="main-section">
            <GamePlayerStatus class="player-status" />
            <GameBoard class="game-board" />
            <div class="below-board">
                <GameCardActionKeys
                    v-if="!roomStore.completed"
                    class="action-keys"
                />
                <GameResult v-else />
            </div>
        </div>
        <div class="side-section move-previews">
            <GameMovePreview :team="PlayerTeam.ALPHA" />
            <GameMovePreview :team="PlayerTeam.BRAVO" />
        </div>
    </div>
</template>

<script lang="ts" setup>
import { useCurrentMoveStore } from '~/stores/CurrentMoveStore';
import { useRoomStore } from '~/stores/RoomStore';
import GameMovePreview from '~/components/game/GameMovePreview.vue';
import { PlayerTeam } from '~/types/PlayerTeam';

const activeCardStore = useCurrentMoveStore();
const roomStore = useRoomStore();
</script>

<style lang="scss">
.game-stage-layout {
    display: grid;
    grid-template-columns: 2fr 4fr 1.25fr;
    gap: 16px;

    position: fixed;
    margin: 0 auto;
    left: 0;
    right: 0;
    max-width: 1200px;
    padding: 20px;
    height: calc(100% - 40px);
    overflow-y: auto;

    &.passing {
        .game-board {
            opacity: 0.5;
        }
    }
}

.below-board {
    text-align: center;
}

.main-section {
    display: flex;
    flex-direction: column;
}

.game-board {
    transition: opacity $default-transition-duration;
    height: 100%;
    width: 100%;
    min-height: 150px;
}

.player-status {
    width: 100%;
    margin-bottom: 8px;
}

.action-keys {
    margin-top: 16px;
}

.side-section {
    z-index: 2;

    &.card-selector-section {
        display: flex;
        align-items: center;
    }

    &.move-previews {
        display: flex;
        flex-direction: column;
        justify-content: center;

        > *:not(:first-child) {
            margin-top: 8px;
        }
    }
}

@include media-breakpoint-down(lg) {
    .game-stage-layout {
        grid-template-columns: 1fr;
        grid-template-rows: 1fr auto;
        padding-top: 10px;
    }

    .card-selector-section {
        display: initial !important;
        order: 1;

        > .card-selector {
            max-width: 600px;
            margin: 0 auto;
        }
    }

    .move-previews {
        display: none !important;
    }
}
</style>
