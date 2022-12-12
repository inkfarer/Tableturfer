<template>
    <div class="card-movement-keys">
        <div class="keypad">
            <GameCardActionKey
                icon="fa6-solid:arrow-rotate-left"
                @click="activeCardStore.previousRotationStep()"
            />
            <GameCardActionKey
                icon="fa6-solid:angle-up"
                @click="activeCardStore.moveUp()"
            />
            <GameCardActionKey
                icon="fa6-solid:arrow-rotate-right"
                @click="activeCardStore.nextRotationStep()"
            />
            <GameCardActionKey
                icon="fa6-solid:angle-left"
                @click="activeCardStore.moveLeft()"
            />
            <GameCardActionKey
                icon="fa6-solid:angle-down"
                @click="activeCardStore.moveDown()"
            />
            <GameCardActionKey
                icon="fa6-solid:angle-right"
                @click="activeCardStore.moveRight()"
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
import { useNuxtApp } from '#imports';
import { useRoomStore } from '~/stores/RoomStore';

const activeCardStore = useCurrentMoveStore();
const roomStore = useRoomStore();
const { $socket } = useNuxtApp();

function placeCard() {
    if (activeCardStore.activeCard == null || roomStore.playerTeam == null || activeCardStore.pass) {
        return;
    }

    $socket.send('ProposeMove', {
        type: 'PlaceCard',
        cardName: activeCardStore.activeCard.name,
        position: activeCardStore.position,
        rotation: activeCardStore.rotation,
        special: activeCardStore.special
    });
    activeCardStore.locked = true;
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
