<template>
    <div class="deck-editor">
        <TtToolbar class="my-2x">
            <TtButton
                inline
                theme="secondary"
                @click="$router.back()"
            >
                <Icon name="fa6-solid:arrow-left" /> {{ $t('deckEditor.back') }}
            </TtButton>
            <TtButton
                inline
                theme="secondary"
                :disabled="changesSaved"
                @click="emit('save')"
            >
                <Icon name="fa6-regular:floppy-disk" /> {{ $t('deckEditor.save') }}
            </TtButton>
        </TtToolbar>
        <div class="details mb-2x mx-4x">
            <TtInput
                v-model="model.name"
                :label="$t('deckEditor.deckName')"
            />
            <div>
                <DataRow :label="$t('deckEditor.squareCount')">
                    {{ squareCount }}
                </DataRow>
            </div>
        </div>
        <div class="card-view">
            <div class="deck-view">
                <template
                    v-for="(card, index) in props.modelValue.cards"
                    :key="`card_${index}`"
                >
                    <Card
                        clickable
                        theme="details"
                        :name="card"
                        :active="selectedCardIndex === index"
                        @click="selectedCardIndex = index"
                    >
                        <template #placeholder>
                            <div class="placeholder-icon">
                                <Icon name="mdi:plus-circle-outline" />
                            </div>
                        </template>
                    </Card>
                </template>
            </div>
            <CardSelect
                :model-value="selectedCardIndex == null ? null : props.modelValue.cards[selectedCardIndex]"
                :disabled="selectedCardIndex == null"
                :disabled-items="modelValue.cards"
                @update:model-value="onCardSelect"
            />
        </div>
    </div>
</template>

<script lang="ts" setup>
import { Deck } from '~/types/DeckList';
import { computed, ref } from '#imports';
import cloneDeep from 'lodash/cloneDeep';
import { CardMap } from '~/helpers/Cards';
import { countCardSquares } from '~/helpers/SquareHelper';
import { useDeckListStore } from '~/stores/DeckListStore';
import isEqual from 'lodash/isEqual';

// todo: validate if saving should be allowed

const emit = defineEmits<{
    (e: 'update:modelValue', value: Deck): void
    (e: 'save'): void
}>();
const props = defineProps<{
    modelValue: Deck
}>();
const model = computed({
    get() {
        return props.modelValue;
    },
    set(value: Deck) {
        emit('update:modelValue', value);
    }
});

const selectedCardIndex = ref<number | null>(null);
function onCardSelect(card: string) {
    if (selectedCardIndex.value == null) {
        return;
    }

    const newCards = cloneDeep(props.modelValue.cards);
    newCards[selectedCardIndex.value] = card;
    model.value.cards = newCards;
}

const squareCount = computed(() => model.value.cards.reduce((result, card) => {
    if (card == null) {
        return result;
    }

    const cardData = CardMap.get(card);
    if (cardData == null) {
        return result;
    }

    return result + countCardSquares(cardData.squares);
}, 0));

const deckListStore = useDeckListStore();
const changesSaved = computed(() => {
    if (props.modelValue.id == null || deckListStore.decks == null) {
        return false;
    }

    const savedDeck = deckListStore.decks[props.modelValue.id];
    return isEqual(savedDeck, props.modelValue);
});
</script>

<style lang="scss" scoped>
.deck-editor {
    max-height: 100vh;
    padding: 0 20px;
    margin: 0 auto;
    max-width: 1440px;
    display: flex;
    flex-direction: column;

    .details {
        display: grid;
        grid-template-columns: 2fr 1fr;
        gap: 16px;
    }
}

.card-view {
    display: grid;
    grid-template-columns: 2fr 3fr;
    gap: 32px;
    overflow: hidden;
    border-top: 2px solid $accent;

    > * {
        padding-top: 8px;
    }
}

.deck-view {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    grid-auto-rows: min-content;
    gap: 8px;
    overflow-y: auto;

    > * {
        aspect-ratio: 0.75;
    }

    .placeholder-icon {
        width: 100%;
        height: 100%;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 3em;
        opacity: 0.5;
    }
}

.card-select {
    overflow-y: auto;
}
</style>
