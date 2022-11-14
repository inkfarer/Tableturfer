import { defineStore } from 'pinia';
import { SocketUser } from '~/types/Socket';

interface RoomStore {
    roomCode: string | null
    users: Record<string, SocketUser>
}

export const useRoomStore = defineStore('room', {
    state: (): RoomStore => ({
        roomCode: null,
        users: {}
    }),
    actions: {
        joinRoom(roomCode: string, users: Record<string, SocketUser>) {
            this.roomCode = roomCode;
            this.users = users;
        },
        addUser(id: string, user: SocketUser) {
            this.users[id] = user;
        },
        removeUser(id: string) {
            delete this.users[id];
        },
        leaveRoom() {
            this.roomCode = null;
            this.users = {};
        }
    }
});
