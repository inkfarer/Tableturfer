<template>
    <div class="game-card-selector">
        <button
            v-for="([name, card]) in CardMap"
            :key="`card_${name}`"
            @click="setActiveCard(card)"
        >
            {{ card.name }}
        </button>
        <button @click="setActiveCard(null)">
            None
        </button>
    </div>
</template>

<script lang="ts" setup>
import { CardMap } from '~/helpers/Cards';
import { Card } from '~/types/Card';
import { useActiveCardStore } from '~/stores/ActiveCardStore';

const activeCardStore = useActiveCardStore();

const setActiveCard = (card: Card | null) => {
    activeCardStore.setActiveCard(card);
};
</script>

<style lang="scss">
.game-card-selector {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    max-height: 400px;
    overflow-y: auto;

    button {
        display: block;

        &:not(:first-child) {
            margin-top: 4px;
        }
    }
}
</style>
