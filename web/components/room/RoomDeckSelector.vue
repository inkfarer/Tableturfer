<template>
    <Overlay v-model="isOpen">
        <h2 class="text-center">{{ $t('room.deckSelect.title') }}</h2>
        <DeckSelect
            :model-value="null"
            disable-unfinished-decks
            class="deck-selector"
            @update:model-value="setDeck($event)"
        />
    </Overlay>
</template>

<script lang="ts" setup>
import { ref, useNuxtApp } from '#imports';
import { useDeckListStore } from '~/stores/DeckListStore';

const isOpen = ref(false);

function open() {
    isOpen.value = true;
}

const deckListStore = useDeckListStore();
const { $socket } = useNuxtApp();

function setDeck(id: string) {
    const selectedDeck = deckListStore.findWithDefault(id);
    if (selectedDeck == null) {
        throw new Error(`Could not find deck "${id}"`);
    }

    $socket.send('SetDeck', { id, cards: selectedDeck.cards });
    isOpen.value = false;
}

defineExpose({
    open
});
</script>

<style lang="scss" scoped>
.deck-selector {
    max-width: 600px;
    margin-top: 12px;
}
</style>
