import { defineStore } from 'pinia';
import { readObjectFromLocalStorage, saveToLocalStorage } from '~/helpers/LocalStorageHelper';
import assign from 'lodash/assign';

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

    hydrate(state) {
        const optionsFromLocalStorage = readObjectFromLocalStorage('userSettings');
        if (optionsFromLocalStorage != null) {
            assign(state, optionsFromLocalStorage);
        }
    },

    actions: {
        save() {
            saveToLocalStorage('userSettings', this.$state);
        }
    }
});
