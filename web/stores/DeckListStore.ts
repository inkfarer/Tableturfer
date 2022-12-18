import { defineStore } from 'pinia';
import { readFromLocalStorage, saveToLocalStorage } from '~/helpers/LocalStorageHelper';
import { Deck } from '~/types/DeckList';
import { v4 as uuidv4 } from 'uuid';

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
        },
        upsert(deck: Deck): string {
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
        save() {
            if (this.decks != null) {
                saveToLocalStorage('deckList', this.decks);
            }
        }
    }
});
