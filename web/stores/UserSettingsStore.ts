import { defineStore } from 'pinia';
import { readObjectFromLocalStorage, saveToLocalStorage } from '~/helpers/LocalStorageHelper';

export interface UserSettingsStore {
    useOnScreenMovementControls: boolean
    useOnScreenRotationAndPlacementControls: boolean
}

export const useUserSettingsStore = defineStore('userSettings', {
    state: (): UserSettingsStore => ({
        useOnScreenMovementControls: false,
        useOnScreenRotationAndPlacementControls: true,
        ...(readObjectFromLocalStorage('userSettings'))
    }),
    actions: {
        save() {
            saveToLocalStorage('userSettings', this.$state);
        }
    }
});
