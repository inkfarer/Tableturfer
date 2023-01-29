import { defineStore } from 'pinia';
import { readObjectFromLocalStorage, saveToLocalStorage } from '~/helpers/LocalStorageHelper';
import assign from 'lodash/assign';
import { useRoomStore } from '~/stores/RoomStore';
import { PlayerTeam } from '~/types/PlayerTeam';
import { useCurrentMoveStore } from '~/stores/CurrentMoveStore';

export interface UserSettingsStore {
    useOnScreenMovementControls: boolean
    useOnScreenRotationAndPlacementControls: boolean
    flipBoardOnBravoTeam: boolean
}

export const useUserSettingsStore = defineStore('userSettings', {
    state: (): UserSettingsStore => ({
        useOnScreenMovementControls: false,
        useOnScreenRotationAndPlacementControls: true,
        flipBoardOnBravoTeam: true,
        ...(readObjectFromLocalStorage('userSettings'))
    }),

    hydrate(state) {
        const optionsFromLocalStorage = readObjectFromLocalStorage('userSettings');
        if (optionsFromLocalStorage != null) {
            assign(state, optionsFromLocalStorage);
        }
    },

    getters: {
        boardFlipped(): boolean {
            const roomStore = useRoomStore();
            return this.flipBoardOnBravoTeam && roomStore.playerTeam === PlayerTeam.BRAVO;
        }
    },

    actions: {
        save() {
            saveToLocalStorage('userSettings', this.$state);
        },
        setBoardFlip(newValue: boolean) {
            if (useRoomStore().started && this.flipBoardOnBravoTeam !== newValue) {
                useCurrentMoveStore().flipPosition();
            }

            this.flipBoardOnBravoTeam = newValue;
        }
    }
});
