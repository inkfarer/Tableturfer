import { defineStore } from 'pinia';

interface RoomStore {
    roomCode: string | null
}

export const useRoomStore = defineStore('room', {
    state: (): RoomStore => ({
        roomCode: null
    })
});
