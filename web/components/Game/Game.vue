<template>
    <div
        class="game-stage-layout"
        :class="{ passing: currentMoveStore.pass }"
    >
        <GameLeavingOverlay ref="leavingOverlay" />
        <div class="side-section card-selector-section">
            <GameCardSelector class="card-selector" />
            <div class="full-deck-view-anchor">
                <GameFullDeckView
                    ref="fullDeckViewElem"
                    v-model:is-open="fullDeckOpen"
                    class="full-deck-view"
                />
            </div>
        </div>
        <div class="side-section beside-card-selector">
            <div class="score-counters">
                <GameTeamScoreCounter :team="PlayerTeam.ALPHA" />
                <GameTeamScoreCounter :team="PlayerTeam.BRAVO" />
            </div>
            <TtToggleButton
                ref="fullDeckToggleButton"
                v-model="fullDeckOpen"
                theme="primary-small"
                class="full-deck-toggle"
            >
                <Icon name="mdi:cards" />
            </TtToggleButton>
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
import GameMovePreview from '~/components/Game/GameMovePreview.vue';
import { PlayerTeam } from '~/types/PlayerTeam';
import { onBeforeRouteLeave, onClickOutside, ref, useNuxtApp } from '#imports';
import { GameBoard, GameLeavingOverlay, TtToggleButton } from '#components';
import useSwipeCardMovement from '~/composables/UseSwipeCardMovement';
import GameFullDeckView from '~/components/Game/GameFullDeckView.vue';

const { $socket } = useNuxtApp();
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

const leavingOverlay = ref<InstanceType<typeof GameLeavingOverlay> | null>(null);
onBeforeRouteLeave(() => {
    if ($socket.isOpen() && leavingOverlay.value && !roomStore.completed) {
        leavingOverlay.value.open();
        return false;
    }
});

const fullDeckOpen = ref(false);
const fullDeckViewElem = ref<InstanceType<typeof GameFullDeckView> | null>(null);
const fullDeckToggleButton = ref<InstanceType<typeof TtToggleButton> | null>(null);
onClickOutside(fullDeckViewElem, () => { fullDeckOpen.value = false; }, { ignore: [fullDeckToggleButton]});

</script>

<style lang="scss">
.game-stage-layout {
    display: grid;
    grid-template-columns: 2.25fr 5em 4fr 1.25fr;
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

    &.beside-card-selector {
        align-self: center;
        justify-self: center;
        display: flex;
        flex-direction: column;
        align-items: center;
        width: 100%;

        > .score-counters {
            display: grid;
            gap: 32px;
            width: 100%;
        }

        > .full-deck-toggle {
            font-size: 2em;
            margin-top: 16px;
        }
    }

    &.card-selector-section {
        display: flex;
        align-items: center;
        z-index: 3;
        position: relative;

        .full-deck-view-anchor {
            position: absolute;
            width: 100%;
            box-sizing: border-box;
            max-height: 100%;
        }
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

    .beside-card-selector {
        order: 1;
        flex-direction: row !important;
        justify-content: center;
        position: relative;
        width: auto !important;

        > .score-counters {
            grid-auto-flow: column dense;
            justify-content: center;
            gap: 8px !important;
            width: auto !important;
        }

        > .full-deck-toggle {
            margin-top: 0 !important;
            position: absolute;
            right: 0;
            transform: translateX(125%);
            padding: 5px 10px;
        }
    }

    .card-selector-section {
        display: initial !important;
        order: 2;

        > .card-selector {
            max-width: 600px;
            margin: 0 auto;
        }

        > .full-deck-view-anchor {
            position: fixed !important;
            bottom: 10px;
            left: 50%;
            transform: translateX(-50%);
            max-height: 90vh !important;
            max-width: min(450px, 95%);
            pointer-events: none;
        }
    }

    .move-previews {
        display: none !important;
    }
}

@include media-breakpoint-down(md) {
    .beside-card-selector {
        > .full-deck-toggle {
            font-size: 1.5em !important;
        }
    }
}
</style>
