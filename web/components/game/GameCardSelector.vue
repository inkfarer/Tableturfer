<template>
    <div class="game-card-selector">
        <button
            v-for="card in deckStore.availableCards"
            :key="`card_${card}`"
            @click="setActiveCard(card)"
        >
            {{ card }}
        </button>
        <button @click="setActiveCard(null)">
            None
        </button>
        <label>
            <input
                v-model="activeCardStore.pass"
                type="checkbox"
            >
            pass?
        </label>
    </div>
</template>

<script lang="ts" setup>
import { CardMap } from '~/helpers/Cards';
import { useActiveCardStore } from '~/stores/ActiveCardStore';
import { useDeckStore } from '~/stores/DeckStore';

const activeCardStore = useActiveCardStore();
const deckStore = useDeckStore();

const setActiveCard = (card: string | null) => {
    activeCardStore.setActiveCard(card == null ? null : CardMap.get(card) ?? null);
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
