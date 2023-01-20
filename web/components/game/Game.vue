<template>
    <div
        class="game-stage-layout"
        :class="{ passing: currentMoveStore.pass }"
    >
        <div class="side-section card-selector-section">
            <GameCardSelector class="card-selector" />
        </div>
        <div class="side-section score-section">
            <GameTeamScoreCounter :team="PlayerTeam.ALPHA" />
            <GameTeamScoreCounter :team="PlayerTeam.BRAVO" />
        </div>
        <div class="main-section">
            <GamePlayerStatus class="player-status" />
            <GameBoard
                ref="gameBoard"
                class="game-board"
                @click="onBoardClick"
                @click.right="onBoardRightClick"
            />
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
import { ref } from '#imports';
import { GameBoard } from '#components';
import useSwipeCardMovement from '~/composables/UseSwipeCardMovement';

const currentMoveStore = useCurrentMoveStore();
const roomStore = useRoomStore();

const gameBoard = ref<InstanceType<typeof GameBoard> | null>(null);
useSwipeCardMovement(gameBoard);

// todo: tapping on the board on mobile executes this.. is this desirable?
function onBoardClick() {
    currentMoveStore.proposeMove();
}

function onBoardRightClick(event: Event) {
    currentMoveStore.nextRotationStep();
    event.preventDefault();
}
</script>

<style lang="scss">
.game-stage-layout {
    display: grid;
    grid-template-columns: 2.25fr 0.5fr 4fr 1.25fr;
    gap: 16px;

    position: fixed;
    margin: 0 auto;
    left: 0;
    right: 0;
    max-width: 1250px;
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

    &.score-section {
        align-self: center;
        justify-self: center;
        width: 100%;
        display: grid;
        gap: 32px;
    }

    &.card-selector-section {
        display: flex;
        align-items: center;
        z-index: 3;
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
        grid-template-rows: 1fr auto auto;
        gap: 8px;
        padding-top: 10px;
        padding-bottom: 10px;
        height: calc(100% - 20px);
    }

    .score-section {
        order: 1;
        grid-auto-flow: column dense;
        justify-content: center;
        gap: 8px !important;
    }

    .card-selector-section {
        display: initial !important;
        order: 2;

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
