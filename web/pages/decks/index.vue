<template>
    <NuxtLayout name="default">
        <template #header>
            <div class="width-cap">
                <h1>{{ $t('deckList.header') }}</h1>
            </div>
        </template>
        <Overlay
            v-model="openRenameOverlay"
            bottom-sheet
        >
            <TtInput
                ref="deckNameInput"
                v-model="deckName"
                :label="$t('deckList.renameOverlay.deckName')"
            />
            <TtButton
                class="mt-1x"
                :disabled="!allowRename"
                @click="applyRename"
            >
                {{ $t('deckList.renameOverlay.save') }}
            </TtButton>
        </Overlay>
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
                class="deck-list-layout mb-2x"
            >
                <DeckSelect
                    v-model="selectedDeck"
                    @update:model-value="showOptions = true"
                />
                <Overlay
                    v-model="showOptions"
                    mobile-only
                    bottom-sheet
                >
                    <div class="deck-options">
                        <TtButton
                            class="mb-2x"
                            :disabled="selectedDeck == null || defaultDeckSelected"
                            @click="editSelected"
                        >
                            {{ $t('deckList.deckAction.edit') }}
                        </TtButton>
                        <TtButton
                            class="mb-1x"
                            :disabled="selectedDeck == null || defaultDeckSelected"
                            @click="renameSelected"
                        >
                            {{ $t('deckList.deckAction.rename') }}
                        </TtButton>
                        <TtButton
                            class="mb-1x"
                            :disabled="selectedDeck == null"
                            @click="copySelected"
                        >
                            {{ $t('deckList.deckAction.copy') }}
                        </TtButton>
                        <TtButton
                            :disabled="selectedDeck == null || defaultDeckSelected"
                            @click="removeSelected"
                        >
                            {{ $t('deckList.deckAction.remove') }}
                        </TtButton>
                    </div>
                </Overlay>
            </div>
        </div>
    </NuxtLayout>
</template>

<script lang="ts" setup>
import { computed, definePageMeta, onMounted, ref, useRouter } from '#imports';
import { useDeckListStore } from '~/stores/DeckListStore';
import { isBlank } from '~/helpers/StringHelper';
import { DEFAULT_DECK_ID } from '~/data/DefaultDeck';
import { ComponentPublicInstance } from 'vue';
import TtInput from '~/components/Tt/TtInput.vue';

definePageMeta({
    layout: false
});

const showOptions = ref(false);
const router = useRouter();
const deckListStore = useDeckListStore();
const selectedDeck = ref<string | null>(null);
const defaultDeckSelected = computed(() => selectedDeck.value === DEFAULT_DECK_ID);

const deckNameInput = ref<ComponentPublicInstance<typeof TtInput> | null>();
const openRenameOverlay = ref(false);
const deckName = ref<string | undefined>();
const allowRename = computed(() => !isBlank(deckName.value));

onMounted(() => {
    deckListStore.load();
});

function removeSelected() {
    if (selectedDeck.value != null) {
        deckListStore.remove(selectedDeck.value);
        deckListStore.save();
    }
    showOptions.value = false;
}

function editSelected() {
    if (selectedDeck.value != null) {
        router.push(`/decks/${selectedDeck.value}/edit`);
    }
    showOptions.value = false;
}

function copySelected() {
    if (selectedDeck.value != null) {
        selectedDeck.value = deckListStore.copy(selectedDeck.value);
        deckListStore.save();
    }
    showOptions.value = false;
}

function renameSelected() {
    if (selectedDeck.value != null) {
        deckName.value = deckListStore.decks?.[selectedDeck.value]?.name;
        openRenameOverlay.value = true;
        deckNameInput.value?.focus();
    }
    showOptions.value = false;
}

function applyRename() {
    if (isBlank(deckName.value)) {
        throw new Error('No name supplied');
    }

    if (selectedDeck.value != null) {
        deckListStore.rename(selectedDeck.value, deckName.value as string);
        deckListStore.save();
        openRenameOverlay.value = false;
    }
}
</script>

<style lang="scss" scoped>
.deck-list-layout {
    display: grid;
    grid-template-columns: 2.2fr 1fr;
    gap: 16px;
    align-items: start;
}

@include media-breakpoint-up(md) {
    .deck-list-layout .deck-options {
        background-color: #262626;
        border-radius: 8px;
        padding: 8px 16px;
    }
}

@include media-breakpoint-down(md) {
    .deck-list-layout {
        grid-template-columns: 1fr;
    }
}
</style>
