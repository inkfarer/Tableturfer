import { Deck } from '~/types/DeckList';

export interface LocalStorageObjectMap {
    deckList: Record<string, Deck>
}

export type LocalStorageStrings = 'username';

export type LocalStorageKeys = keyof LocalStorageObjectMap | LocalStorageStrings;
export type LocalStorageDataMap = (LocalStorageObjectMap & { [Key in LocalStorageStrings]: string });
