<template>
    <div v-if="loading">
        Loading...
    </div>
    <div v-else-if="deck == null">
        Deck not found!
    </div>
    <DeckEditor
        v-else
        v-model="deck"
        @save="onSave"
    />
</template>

<script lang="ts" setup>
import { useDeckListStore } from '~/stores/DeckListStore';
import { computed, definePageMeta, onMounted, ref, useHead, useI18n, useRoute, useState, watch } from '#imports';
import { Deck } from '~/types/DeckList';
import cloneDeep from 'lodash/cloneDeep';
import { isBlank } from '~/helpers/StringHelper';

definePageMeta({
    layout: false
});

const i18n = useI18n();
const route = useRoute();
const deckListStore = useDeckListStore();
const loading = ref(true);
const deck = useState<Deck | null>('deckToEdit', () => null);

useHead({
    title: computed(() => isBlank(deck.value?.name)
        ? i18n.t('deckEditor.title.editing.default')
        : i18n.t('deckEditor.title.editing.withName', { name: deck.value?.name }))
});

onMounted(() => {
    deckListStore.load();

    watch(() => route.params.id, newValue => {
        loading.value = true;
        deck.value = cloneDeep(deckListStore.decks?.[newValue as string] ?? null);
        loading.value = false;
    }, { immediate: true });
});

function onSave() {
    if (deck.value == null) {
        throw new Error('Deck not yet loaded');
    }

    deckListStore.upsert({
        ...deck.value,
        id: route.params.id as string
    });
    deckListStore.save();
}
</script>
