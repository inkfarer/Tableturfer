import { Deck } from '~/types/DeckList';

export interface LocalStorageItemMap {
    deckList: Record<string, Deck>
}
