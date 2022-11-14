import { defineStore } from 'pinia';
import { SocketMessageMap, SocketUser } from '~/types/Socket';

interface RoomStore {
    id: string | null
    roomCode: string | null
    owner: string | null
    users: Record<string, SocketUser>
}

export const useRoomStore = defineStore('room', {
    state: (): RoomStore => ({
        id: null,
        roomCode: null,
        owner: null,
        users: {}
    }),
    getters: {
        isRoomOwner: state => state.id === state.owner
    },
    actions: {
        joinRoom(message: SocketMessageMap['Welcome']) {
            this.id = message.id;
            this.roomCode = message.roomCode;
            this.users = message.users;
            this.owner = message.owner;
        },
        addUser(id: string, user: SocketUser) {
            this.users[id] = user;
        },
        removeUser(id: string) {
            delete this.users[id];
        },
        leaveRoom() {
            this.roomCode = null;
            this.owner = null;
            this.users = {};
        }
    }
});
