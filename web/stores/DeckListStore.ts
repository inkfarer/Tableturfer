import { defineStore } from 'pinia';
import { readObjectFromLocalStorage, saveToLocalStorage } from '~/helpers/LocalStorageHelper';
import { Deck, NewDeck } from '~/types/DeckList';
import { v4 as uuidv4 } from 'uuid';
import cloneDeep from 'lodash/cloneDeep';
import { createDefaultDeck, DEFAULT_DECK_ID } from '~/data/DefaultDeck';

interface DeckListStore {
    decks: Record<string, Deck> | null
}

export const useDeckListStore = defineStore('deckList', {
    state: (): DeckListStore => ({
        decks: null
    }),
    getters: {
        findWithDefault() {
            return (id: string): Deck | undefined =>
                id === DEFAULT_DECK_ID ? createDefaultDeck(this.$i18n) : this.decks?.[id];
        }
    },
    actions: {
        load() {
            this.decks = readObjectFromLocalStorage('deckList') ?? {};
        },
        upsert(deck: NewDeck): string {
            if (this.decks == null) {
                this.load();
            }

            const id = deck.id ?? uuidv4();
            // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
            this.decks![id] = {
                ...deck,
                id
            };

            return id;
        },
        copy(id: string): string {
            if (this.decks == null) {
                throw new Error('Deck list not loaded');
            }

            const deck = this.findWithDefault(id);
            if (deck == null) {
                throw new Error(`Deck ${id} not found`);
            }

            return this.upsert({
                ...(cloneDeep(deck)),
                name: this.$i18n.t('deckName.copyName', { name: deck.name }),
                id: null
            });
        },
        rename(id: string, name: string) {
            if (this.decks == null) {
                throw new Error('Deck list not loaded');
            }

            const deck = this.decks[id];
            if (deck == null) {
                throw new Error(`Deck ${id} not found`);
            }

            deck.name = name;
        },
        remove(id: string) {
            if (this.decks == null) {
                throw new Error('Deck list not loaded');
            }

            delete this.decks[id];
        },
        save() {
            if (this.decks != null) {
                saveToLocalStorage('deckList', this.decks);
            }
        }
    }
});
