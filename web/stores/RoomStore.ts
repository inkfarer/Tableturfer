import { defineStore } from 'pinia';
import { SocketMessageMap, SocketUser } from '~/types/socket/SocketEvent';
import { PlayerTeam } from '~/types/PlayerTeam';

interface RoomStore {
    id: string | null
    roomCode: string | null
    owner: string | null
    opponent: string | null
    users: Record<string, SocketUser>
}

export const useRoomStore = defineStore('room', {
    state: (): RoomStore => ({
        id: null,
        roomCode: null,
        owner: null,
        opponent: null,
        users: {}
    }),
    getters: {
        isRoomOwner: state => state.id === state.owner,
        isOpponent: state => state.id === state.opponent,
        playerTeam() {
            if (this.isRoomOwner) {
                return PlayerTeam.ALPHA;
            } else if (this.isOpponent) {
                return PlayerTeam.BRAVO;
            } else {
                return null;
            }
        }
    },
    actions: {
        joinRoom(message: SocketMessageMap['Welcome']) {
            this.id = message.id;
            this.roomCode = message.roomCode;
            this.users = message.users;
            this.owner = message.owner;
            this.opponent = message.opponent;
        },
        addUser(id: string, user: SocketUser) {
            this.users[id] = user;
        },
        removeUser(id: string) {
            delete this.users[id];
        },
        leaveRoom() {
            this.$reset();
        }
    }
});
