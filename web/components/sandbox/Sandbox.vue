<template>
    <div class="sandbox-layout">
        <Alert
            class="wip-alert"
            theme="info"
        >
            {{ $t('sandbox.wipAlert') }}
        </Alert>
        <SandboxCardSelector
            class="card-selector"
        />
        <GameBoard
            ref="gameBoard"
            class="game-board"
            @click="onBoardClick"
            @click.right="onBoardRightClick"
        />
    </div>
</template>

<script lang="ts" setup>
import { ref } from '#imports';
import { GameBoard } from '#components';
import useSwipeCardMovement from '~/composables/UseSwipeCardMovement';
import { useCurrentMoveStore } from '~/stores/CurrentMoveStore';
import { useGameBoardStore } from '~/stores/GameBoardStore';
import { PlayerTeam } from '~/types/PlayerTeam';

const currentMoveStore = useCurrentMoveStore();
const gameBoardStore = useGameBoardStore();

const gameBoard = ref<InstanceType<typeof GameBoard> | null>(null);
useSwipeCardMovement(gameBoard);

function onBoardClick() {
    if (currentMoveStore.activeCard == null) {
        return;
    }

    if (gameBoardStore.isPlaceable(currentMoveStore.position, currentMoveStore.activeCard.squares)) {
        gameBoardStore.applyMoves({
            [PlayerTeam.ALPHA]: {
                type: 'PlaceCard',
                cardName: currentMoveStore.activeCard.name,
                position: currentMoveStore.position,
                rotation: currentMoveStore.rotation,
                special: false
            },
            // todo: the api for this should be nicer than me having to fudge a fake move in here
            [PlayerTeam.BRAVO]: {
                type: 'Pass',
                cardName: 'none'
            }
        });
    }
}

function onBoardRightClick(event: Event) {
    currentMoveStore.nextRotationStep();
    event.preventDefault();
}

</script>

<style lang="scss" scoped>
.sandbox-layout {
    position: fixed;
    margin: 0 auto;
    left: 0;
    right: 0;
    max-width: 1200px;
    padding: 20px;
    height: calc(100% - 40px);
    display: grid;
    grid-template-columns: 2fr 3fr;
    gap: 16px;

    > .wip-alert {
        grid-column: span 2;
    }

    > .card-selector {
        overflow-y: auto;
    }
}
</style>
