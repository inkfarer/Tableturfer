<template>
    <div class="card-movement-keys">
        <div class="keypad">
            <GameCardActionKey
                icon="fa6-solid:arrow-rotate-left"
                @click="currentMoveStore.previousRotationStep()"
            />
            <GameCardActionKey
                icon="fa6-solid:angle-up"
                @click="currentMoveStore.moveUp()"
            />
            <GameCardActionKey
                icon="fa6-solid:arrow-rotate-right"
                @click="currentMoveStore.nextRotationStep()"
            />
            <GameCardActionKey
                icon="fa6-solid:angle-left"
                @click="currentMoveStore.moveLeft()"
            />
            <GameCardActionKey
                icon="fa6-solid:angle-down"
                @click="currentMoveStore.moveDown()"
            />
            <GameCardActionKey
                icon="fa6-solid:angle-right"
                @click="currentMoveStore.moveRight()"
            />
        </div>

        <div class="extra-buttons">
            <GameCardActionKey
                :text="$t('game.placeCard')"
                @click="placeCard"
            />
        </div>
    </div>
</template>

<script lang="ts" setup>
import { useCurrentMoveStore } from '~/stores/CurrentMoveStore';

const currentMoveStore = useCurrentMoveStore();

function placeCard() {
    currentMoveStore.proposeMove();
}
</script>

<style lang="scss" scoped>
.card-movement-keys {
    display: flex;
    flex-direction: column;
    align-items: center;

    .keypad {
        display: grid;
        grid-template-columns: repeat(3, 1fr);
        gap: 4px;
    }

    .extra-buttons {
        margin-top: 4px;
        display: grid;
        grid-template-columns: 1fr;
        gap: 4px;
        justify-items: center;
    }
}
</style>
