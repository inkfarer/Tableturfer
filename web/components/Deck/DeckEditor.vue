<template>
    <div class="deck-editor">
        <TtToolbar class="my-2x toolbar">
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
                :disabled="changesSaved || !hasName"
                @click="emit('save')"
            >
                <Icon name="fa6-regular:floppy-disk" /> {{ $t('deckEditor.save') }}
            </TtButton>
        </TtToolbar>
        <div class="card-view">
            <div>
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
                            @click="selectedCardIndex = index; cardSelectOpen = true"
                        >
                            <template #placeholder>
                                <div class="placeholder-icon">
                                    <Icon name="mdi:plus-circle-outline" />
                                </div>
                            </template>
                        </Card>
                    </template>
                </div>
            </div>
            <Overlay
                v-model="cardSelectOpen"
                mobile-only
                bottom-sheet
            >
                <CardSelect
                    :model-value="selectedCardIndex == null ? null : props.modelValue.cards[selectedCardIndex]"
                    :disabled="selectedCardIndex == null"
                    :disabled-items="modelValue.cards"
                    @update:model-value="onCardSelect"
                />
            </Overlay>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { NewDeck } from '~/types/DeckList';
import { computed, ref } from '#imports';
import cloneDeep from 'lodash/cloneDeep';
import { CardMap } from '~/helpers/Cards';
import { countCardSquares } from '~/helpers/SquareHelper';
import { useDeckListStore } from '~/stores/DeckListStore';
import isEqual from 'lodash/isEqual';
import { isBlank } from '~/helpers/StringHelper';

const emit = defineEmits<{
    (e: 'update:modelValue', value: NewDeck): void
    (e: 'save'): void
}>();
const props = defineProps<{
    modelValue: NewDeck
}>();
const model = computed({
    get() {
        return props.modelValue;
    },
    set(value: NewDeck) {
        emit('update:modelValue', value);
    }
});

const cardSelectOpen = ref(false);
const selectedCardIndex = ref<number | null>(null);
function onCardSelect(card: string) {
    if (selectedCardIndex.value == null) {
        return;
    }

    const newCards = cloneDeep(props.modelValue.cards);
    newCards[selectedCardIndex.value] = card;
    model.value.cards = newCards;
    cardSelectOpen.value = false;
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

const hasName = computed(() => !isBlank(props.modelValue.name));
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
        gap: 16px;
        padding-top: 8px;
    }

    .toolbar {
        margin-bottom: 0;
    }
}

.card-view {
    display: grid;
    grid-template-columns: 2fr 3fr;
    gap: 32px;
    overflow: hidden;

    > * {
        padding-top: 8px;
        overflow-y: auto;
    }
}

.deck-view {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(125px, auto));
    grid-auto-rows: min-content;
    gap: 8px;

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
    padding-top: 8px;
}

@include media-breakpoint-down(md) {
    .card-view {
        grid-template-columns: 1fr;
    }
}
</style>
