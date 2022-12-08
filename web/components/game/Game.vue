<template>
    <div class="game-stage-layout">
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

<style lang="scss">
.game-stage-layout {
    display: grid;
    grid-template-columns: 2fr auto 1fr;
    gap: 16px;
}

.game-board {
    z-index: -1;
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
