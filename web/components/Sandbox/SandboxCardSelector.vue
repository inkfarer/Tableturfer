<template>
    <div class="sandbox-card-selector">
        <Card
            v-for="card in deck"
            :key="`card_${card}`"
            v-long-press
            :name="card"
            :active="activeCardStore.activeCard?.name === card"
            clickable
            theme="miniature"
            class="card"
            @short-press="selectCard(card)"
        />
    </div>
</template>

<script lang="ts" setup>
import { CardMap } from '~/helpers/Cards';
import { useCurrentMoveStore } from '~/stores/CurrentMoveStore';
import { DEFAULT_DECK_CARDS } from '~/data/DefaultDeck';

const activeCardStore = useCurrentMoveStore();
const deck = DEFAULT_DECK_CARDS;

const selectCard = (card: string) => {
    if (card != null && activeCardStore.activeCard?.name === card) {
        activeCardStore.setActiveCard(null);
    } else {
        activeCardStore.setActiveCard(CardMap.get(card) ?? null);
    }
};
</script>

<style lang="scss" scoped>
.sandbox-card-selector {
    position: relative;
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 8px;
}
</style>
