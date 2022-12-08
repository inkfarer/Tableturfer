<template>
    <div class="card-movement-keys">
        <div class="keypad">
            <TtButton @click="activeCardStore.previousRotationStep()">
                <Icon name="fa6-solid:arrow-rotate-left" />
            </TtButton>
            <TtButton @click="activeCardStore.moveUp()">
                <Icon name="fa6-solid:angle-up" />
            </TtButton>
            <TtButton @click="activeCardStore.nextRotationStep()">
                <Icon name="fa6-solid:arrow-rotate-right" />
            </TtButton>
            <TtButton @click="activeCardStore.moveLeft()">
                <Icon name="fa6-solid:angle-left" />
            </TtButton>
            <TtButton @click="activeCardStore.moveDown()">
                <Icon name="fa6-solid:angle-down" />
            </TtButton>
            <TtButton @click="activeCardStore.moveRight()">
                <Icon name="fa6-solid:angle-right" />
            </TtButton>
        </div>

        <div class="extra-buttons">
            <TtButton @click="placeCard">{{ $t('game.placeCard') }}</TtButton>
            <TtButton
                v-if="roomStore.completed && roomStore.isRoomOwner"
                @click="returnToRoom"
            >
                {{ $t('game.returnToRoom') }}
            </TtButton>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { useActiveCardStore } from '~/stores/ActiveCardStore';
import { useNuxtApp } from '#imports';
import { useRoomStore } from '~/stores/RoomStore';

const activeCardStore = useActiveCardStore();
const roomStore = useRoomStore();
const { $socket } = useNuxtApp();

function returnToRoom() {
    $socket.send('ReturnToRoom');
}

function placeCard() {
    if (activeCardStore.activeCard == null || roomStore.playerTeam == null) {
        return;
    }

    if (!activeCardStore.pass) {
        $socket.send('ProposeMove', {
            type: 'PlaceCard',
            cardName: activeCardStore.activeCard.name,
            position: activeCardStore.position,
            rotation: activeCardStore.rotation,
            special: activeCardStore.special
        });
    } else {
        $socket.send('ProposeMove', {
            type: 'Pass',
            cardName: activeCardStore.activeCard.name
        });
    }
    activeCardStore.setActiveCard(null);
    activeCardStore.special = false;
    activeCardStore.pass = false;
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
