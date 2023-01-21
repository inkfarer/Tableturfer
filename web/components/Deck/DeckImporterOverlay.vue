<template>
    <Overlay v-model="isOpen">
        <div class="deck-importer-overlay">
            <h2>{{ $t('deckImporter.title') }}</h2>
            <TtDetails>
                <summary>{{ $t('deckImporter.sourceList.title') }}</summary>
                <ul>
                    <li>
                        <NuxtLink href="https://schazz-guy.itch.io/tableturftooltip">
                            {{ $t('deckImporter.sourceList.tooltip') }}
                        </NuxtLink>
                    </li>
                    <li>
                        <NuxtLink href="https://tableturf.koishi.top/">
                            {{ $t('deckImporter.sourceList.koishi.name') }}
                        </NuxtLink>
                        {{ $t('deckImporter.sourceList.koishi.note') }}
                    </li>
                    <li>
                        <NuxtLink href="https://tableturf.andriocelos.net/">
                            {{ $t('deckImporter.sourceList.andriocelos') }}
                        </NuxtLink>
                    </li>
                </ul>
            </TtDetails>
            <Alert
                v-if="importErrorMessageKey != null"
                theme="error"
                class="mt-1x"
            >
                {{ $t(importErrorMessageKey) }}
            </Alert>
            <Alert
                v-else-if="lastImportedDeck != null"
                theme="success"
                class="mt-1x"
            >
                {{ $t('deckImporter.importSuccess', lastImportedDeck.cards.filter(card => card != null).length) }}
            </Alert>
            <TtTextarea
                v-model="deckData"
                :label="$t('deckImporter.dataInputLabel')"
                class="mt-1x"
            />
            <TtButton
                class="mt-1x"
                @click="onImport"
            >
                {{ $t('deckImporter.submit') }}
            </TtButton>
        </div>
    </Overlay>
</template>

<script lang="ts" setup>
import { parseDeck, ref, TranslatableError, useI18n } from '#imports';
import { useDeckListStore } from '~/stores/DeckListStore';
import { isBlank } from '~/helpers/StringHelper';
import { Deck } from '~/types/DeckList';

const isOpen = ref(false);
function open() {
    isOpen.value = true;
    importErrorMessageKey.value = null;
    lastImportedDeck.value = null;
}

defineExpose({ open });

const i18n = useI18n();
const deckListStore = useDeckListStore();
const deckData = ref('');
const importErrorMessageKey = ref<string | null>(null);
const lastImportedDeck = ref<Deck | null>(null);

function onImport() {
    try {
        lastImportedDeck.value = null;
        const parsedDeck = parseDeck(deckData.value);
        if (isBlank(parsedDeck.name)) {
            parsedDeck.name = i18n.t('deckImporter.defaultDeckName');
        }
        const deckId = deckListStore.upsert(parsedDeck);
        deckListStore.save();

        importErrorMessageKey.value = null;
        lastImportedDeck.value = {
            ...parsedDeck,
            id: deckId
        };
    } catch (e) {
        if (e instanceof TranslatableError) {
            importErrorMessageKey.value = e.translationKey;
            if (e.cause != null) {
                console.error('Importing deck failed with cause:', e.cause);
            }
        } else {
            importErrorMessageKey.value = 'deckParser.error.unknown';
        }
    }
}
</script>

<style lang="scss" scoped>
.deck-importer-overlay {
    width: 500px;
    text-align: center;

    > details {
        margin-top: 10px;

        ul {
            margin: 0;
        }
    }
}
</style>
