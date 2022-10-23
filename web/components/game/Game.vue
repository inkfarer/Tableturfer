<template>
    <div class="game-stage-layout">
        <GameBoard />
        <div class="side-section">
            <GameCardSelector />
            <GameCardMovementKeys class="movement-keys" />
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

.movement-keys {
    margin-top: 10px;
}
</style>
