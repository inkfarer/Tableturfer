<template>
    <div
        class="game-stage-layout"
        :class="{ passing: activeCardStore.pass }"
    >
        <div class="side-section">
            <GameCardSelector class="card-selector" />
        </div>
        <div class="main-section">
            <GamePlayerStatus class="player-status" />
            <GameBoard class="game-board" />
            <GameCardActionKeys class="action-keys" />
        </div>
        <div class="side-section">

        </div>
    </div>
</template>

<script lang="ts" setup>
import { useActiveCardStore } from '~/stores/ActiveCardStore';

const activeCardStore = useActiveCardStore();
</script>

<style lang="scss">
.game-stage-layout {
    display: grid;
    grid-template-columns: 2fr auto 1fr;
    gap: 16px;

    &.passing {
        .game-board {
            opacity: 0.5;
        }
    }
}

.game-board {
    z-index: -1;
    transition: opacity $default-transition-duration;
}

.main-section {
    display: flex;
    flex-direction: column;
    align-items: center;
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
    display: flex;
    align-items: center;
}

.top-margin {
    margin-top: 10px;
}
</style>
