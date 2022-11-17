<template>
    <div class="game-stage-layout">
        <div class="side-section">
            <GameMapSelector />
            <GameCardSelector class="top-margin" />
        </div>
        <GameBoard class="game-board" />
        <div class="side-section">
            <GameDebugInfo />
            <GameCardActionKeys class="top-margin" />
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

.top-margin {
    margin-top: 10px;
}
</style>
