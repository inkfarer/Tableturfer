<template>
    <div class="game-stage-layout">
        <div class="side-section">
            <GameMapSelector />
        </div>
        <GameBoard />
        <div class="side-section">
            <GameCardSelector />
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

.action-keys {
    margin-top: 10px;
}
</style>
