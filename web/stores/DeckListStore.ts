import { defineStore } from 'pinia';
import { readFromLocalStorage } from '~/helpers/LocalStorageHelper';
import { Deck } from '~/types/DeckList';

interface DeckListStore {
    decks: Record<string, Deck> | null
}

export const useDeckListStore = defineStore('deckList', {
    state: (): DeckListStore => ({
        decks: null
    }),
    actions: {
        load() {
            this.decks = readFromLocalStorage('deckList') ?? {};
        }
    }
});
