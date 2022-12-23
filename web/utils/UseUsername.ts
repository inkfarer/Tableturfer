import { useState } from '#imports';
import { readStringFromLocalStorage, saveToLocalStorage } from '~/helpers/LocalStorageHelper';

export const useUsername = () => useState<string | null>('username', () => null);

export function initUsernameAfterLoad() {
    if (useUsername().value == null) {
        useUsername().value = readStringFromLocalStorage('username');
    }
}

export function saveUsername() {
    const username = useUsername().value;
    if (username != null) {
        saveToLocalStorage('username', username);
    }
}
