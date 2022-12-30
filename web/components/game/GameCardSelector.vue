<template>
    <div class="game-card-selector">
        <Card
            v-for="card in deckStore.availableCards"
            :key="`card_${card}`"
            :name="card"
            :active="activeCardStore.activeCard?.name === card"
            :clickable="!activeCardStore.locked"
            :team="roomStore.playerTeam"
            theme="miniature"
            class="card"
            @click="selectCard(card)"
        />
        <TtToggleButton
            :model-value="activeCardStore.pass"
            :disabled="activeCardStore.locked"
            class="action-button"
            @update:model-value="activeCardStore.setPass($event)"
        >
            {{ $t('game.pass') }}
        </TtToggleButton>
        <TtToggleButton
            :model-value="activeCardStore.special"
            :disabled="activeCardStore.locked"
            class="action-button"
            @update:model-value="activeCardStore.setSpecial($event)"
        >
            {{ $t('game.special') }}
        </TtToggleButton>
    </div>
</template>

<script lang="ts" setup>
import { CardMap } from '~/helpers/Cards';
import { useCurrentMoveStore } from '~/stores/CurrentMoveStore';
import { useDeckStore } from '~/stores/DeckStore';
import { useNuxtApp } from '#imports';
import { useRoomStore } from '~/stores/RoomStore';

const roomStore = useRoomStore();
const activeCardStore = useCurrentMoveStore();
const deckStore = useDeckStore();
const { $socket } = useNuxtApp();

const selectCard = (card: string) => {
    if (activeCardStore.pass) {
        activeCardStore.setActiveCard(CardMap.get(card) ?? null);
        $socket.send('ProposeMove', {
            type: 'Pass',
            cardName: card
        });
        activeCardStore.locked = true;
    } else {
        if (card != null && activeCardStore.activeCard?.name === card) {
            activeCardStore.setActiveCard(null);
        } else {
            activeCardStore.setActiveCard(CardMap.get(card) ?? null);
        }
    }
};
</script>

<style lang="scss">
.game-card-selector {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 8px;

    //> .card {
    //    aspect-ratio: 3 / 4;
    //}
}

@include media-breakpoint-down(lg) {
    .game-card-selector {
        grid-template-columns: repeat(4, 1fr);

        > .card {
            //min-height: 100px;
        }
    }

    .action-button {
        grid-column: span 2;
        height: max-content;
    }
}

@include media-breakpoint-down(sm) {
    .action-button {
        font-size: 1em !important;
    }
}
</style>
