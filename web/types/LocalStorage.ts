import { Deck } from '~/types/DeckList';
import { UserSettingsStore } from '~/stores/UserSettingsStore';

export interface LocalStorageObjectMap {
    deckList: Record<string, Deck>
    userSettings: UserSettingsStore
}

export type LocalStorageStrings = 'username';

export type LocalStorageKeys = keyof LocalStorageObjectMap | LocalStorageStrings;
export type LocalStorageDataMap = (LocalStorageObjectMap & { [Key in LocalStorageStrings]: string });
