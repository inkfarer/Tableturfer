import { LocalStorageItemMap } from '~/types/LocalStorage';

export function readFromLocalStorage<T extends keyof LocalStorageItemMap>(key: T): LocalStorageItemMap[T] | null {
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

export function saveToLocalStorage<T extends keyof LocalStorageItemMap>(key: T, data: LocalStorageItemMap[T]) {
    window.localStorage.setItem(key, JSON.stringify(data));
}
