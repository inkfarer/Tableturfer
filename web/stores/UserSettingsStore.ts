import { defineStore } from 'pinia';
import { readObjectFromLocalStorage, saveToLocalStorage } from '~/helpers/LocalStorageHelper';

export interface UserSettingsStore {
    useOnScreenControls: boolean
}

const defaultUserSettings: UserSettingsStore = Object.freeze({
    useOnScreenControls: false
});

export const useUserSettingsStore = defineStore('userSettings', {
    state: (): UserSettingsStore => ({
        ...defaultUserSettings,
        ...(readObjectFromLocalStorage('userSettings'))
    }),
    actions: {
        save() {
            saveToLocalStorage('userSettings', this.$state);
        }
    }
});
