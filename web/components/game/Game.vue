<template>
    <div class="game-stage-layout">
        <div class="side-section">
            <GameMapSelector />
            <GameCardSelector style="margin-top: 10px;" />
        </div>
        <GameBoard class="game-board" />
        <div class="side-section">
            <GameDebugInfo />
            <GameCardActionKeys class="action-keys" />
        </div>
    </div>
</template>

<style lang="scss">
.game-stage-layout {
    max-width: 1000px;
    display: grid;
    grid-template-columns: 1fr auto 1fr;
    gap: 10px;
}

.game-board {
    z-index: -1;
}

.side-section {
    z-index: 2;
}

.action-keys {
    margin-top: 10px;
}
</style>
