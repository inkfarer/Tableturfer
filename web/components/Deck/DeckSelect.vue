<template>
    <div class="deck-select">
        <div
            v-for="deck in decks"
            :key="`deck_${deck.id}`"
            class="deck"
            :class="{
                selected: props.modelValue === deck.id,
                disabled: props.disableUnfinishedDecks && deck.cards.some(card => card == null)
            }"
            @click="onDeckClick(deck)"
        >
            {{ deck.name }}
        </div>
    </div>
</template>

<script lang="ts" setup>
import { useDeckListStore } from '~/stores/DeckListStore';
import { computed, useI18n } from '#imports';
import { createDefaultDeck, DEFAULT_DECK_ID } from '~/data/DefaultDeck';
import { Deck } from '~/types/DeckList';

const emit = defineEmits<{
    (e: 'update:modelValue', value: string): void
}>();
const props = withDefaults(defineProps<{
    modelValue: string | null
    disableUnfinishedDecks?: boolean
}>(), {
    disableUnfinishedDecks: false
});

const i18n = useI18n();

const deckListStore = useDeckListStore();
const decks = computed(() => {
    return ({
        [DEFAULT_DECK_ID]: createDefaultDeck(i18n),
        ...deckListStore.decks
    });
});

function onDeckClick(deck: Deck) {
    if (!props.disableUnfinishedDecks || deck.cards.every(card => card != null)) {
        emit('update:modelValue', deck.id);
    }
}
</script>

<style lang="scss" scoped>
.deck-select {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(225px, auto));
    gap: 8px;

    > .deck {
        border: 2px solid $accent;
        background-color: $accent-a10;
        text-align: center;
        border-radius: 8px;
        padding: 12px 0;
        font-size: 1.25em;
        font-weight: 500;
        transition: background-color $default-transition-duration;
        cursor: pointer;
        user-select: none;

        &.selected {
            background-color: $accent-a35 !important;
        }

        &.disabled {
            opacity: 0.5;
            cursor: initial;
        }

        &:not(.disabled) {
            &:hover {
                background-color: $accent-a20;
            }

            &:active {
                background-color: $accent-a35;
            }
        }
    }
}
</style>
