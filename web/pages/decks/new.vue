<template>
    <DeckEditor
        v-model="deck"
        @save="onSave"
    />
</template>

<script lang="ts" setup>
import { definePageMeta, useI18n, useState } from '#imports';
import { DECK_SIZE } from '~/data/Constants';
import { Deck } from '~/types/DeckList';
import { useDeckListStore } from '~/stores/DeckListStore';

const i18n = useI18n();

definePageMeta({
    layout: false
});

const deck = useState<Deck>('newDeck', () => ({
    id: null,
    name: i18n.t('deckName.defaultName'),
    cards: new Array(DECK_SIZE).fill(null)
}));

const deckListStore = useDeckListStore();

function onSave() {
    deck.value.id = deckListStore.upsert(deck.value);
    deckListStore.save();
}
</script>
