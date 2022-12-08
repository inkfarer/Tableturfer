<template>
    <div class="game-card-selector">
        <GameCardPreview
            v-for="card in deckStore.availableCards"
            :key="`card_${card}`"
            :name="card"
            @click="selectCard(card)"
        />
        <TtToggleButton
            :model-value="activeCardStore.pass"
            :disabled="activeCardStore.locked"
            @update:model-value="activeCardStore.setPass($event)"
        >
            {{ $t('game.pass') }}
        </TtToggleButton>
        <TtToggleButton
            :model-value="activeCardStore.special"
            :disabled="activeCardStore.locked"
            @update:model-value="activeCardStore.setSpecial($event)"
        >
            {{ $t('game.special') }}
        </TtToggleButton>
    </div>
</template>

<script lang="ts" setup>
import { CardMap } from '~/helpers/Cards';
import { useActiveCardStore } from '~/stores/ActiveCardStore';
import { useDeckStore } from '~/stores/DeckStore';
import GameCardPreview from '~/components/game/GameCardPreview.vue';
import { useNuxtApp } from '#imports';

const activeCardStore = useActiveCardStore();
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
}
</style>
