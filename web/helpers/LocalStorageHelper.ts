import {
    LocalStorageDataMap,
    LocalStorageKeys,
    LocalStorageObjectMap,
    LocalStorageStrings
} from '~/types/LocalStorage';

export function readStringFromLocalStorage(key: LocalStorageStrings): string | null {
    if (typeof window === 'undefined') {
        return null;
    }

    return window.localStorage.getItem(key);
}

export function readObjectFromLocalStorage<T extends keyof LocalStorageObjectMap>(key: T): LocalStorageObjectMap[T] | null {
    if (typeof window === 'undefined') {
        // Must be SSR, nothing to return
        return null;
    }

    const rawData = window.localStorage.getItem(key);
    if (rawData == null) {
        return null;
    }

    try {
        // todo: schema validation?
        //  to guard against clients or buggy code saving bad data into local storage
        return JSON.parse(rawData);
    } catch (e) {
        console.error(`Removing key "${key}" from local storage as it is not valid JSON`, e);
        window.localStorage.removeItem(key);
        return null;
    }
}

export function saveToLocalStorage<T extends LocalStorageKeys>(key: T, data: LocalStorageDataMap[T]) {
    if (typeof window === 'undefined') {
        throw new Error('Tried to save data to local storage during SSR?');
    }

    const normalizedData = typeof data === 'string' ? data : JSON.stringify(data);
    window.localStorage.setItem(key, normalizedData);
}
