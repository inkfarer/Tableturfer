<template>
    <NuxtLayout name="default">
        <template #header>
            <div class="width-cap">
                <h1>Decks</h1>
            </div>
        </template>
        <div class="width-cap">
            <Alert
                theme="info"
                class="my-2x"
            >
                {{ $t('deckList.localStorageMessage') }}
            </Alert>
            <TtToolbar class="mb-2x">
                <TtLinkButton
                    to="/decks/new"
                    inline
                    theme="secondary"
                >
                    <Icon name="fa6-solid:plus" /> {{ $t('deckList.createNew') }}
                </TtLinkButton>
            </TtToolbar>
            <p
                v-if="deckListStore.decks == null"
                class="text-center"
            >
                {{ $t('deckList.loading') }}
            </p>
            <div
                v-else
                class="deck-list-layout"
            >
                hello
            </div>
        </div>
    </NuxtLayout>
</template>

<script lang="ts" setup>
import { definePageMeta, onMounted } from '#imports';
import { useDeckListStore } from '~/stores/DeckListStore';

definePageMeta({
    layout: false
});

const deckListStore = useDeckListStore();

onMounted(() => {
    deckListStore.load();
});
</script>
