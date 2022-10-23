<template>
    <div class="game-stage-layout">
        <GameBoard />
        <div class="side-section">
            <GameCardSelector />
            <GameCardActionKeys class="action-keys" />
        </div>
        <div class="side-section">
            <GameMapSelector />
        </div>
    </div>
</template>

<style lang="scss">
.game-stage-layout {
    display: flex;
}

.side-section {
    margin-left: 10px;
}

.action-keys {
    margin-top: 10px;
}
</style>
