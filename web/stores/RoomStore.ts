import { defineStore } from 'pinia';
import { SocketMessageMap, SocketUser } from '~/types/socket/SocketEvent';
import { PlayerTeam } from '~/types/PlayerTeam';
import { TURN_COUNT } from '~/data/Constants';

interface RoomStore {
    id: string | null
    roomCode: string | null
    owner: string | null
    opponent: string | null
    users: Record<string, SocketUser>
    started: boolean
    remainingTurns: number
}

export const useRoomStore = defineStore('room', {
    state: (): RoomStore => ({
        id: null,
        roomCode: null,
        owner: null,
        opponent: null,
        users: {},
        started: false,
        remainingTurns: TURN_COUNT
    }),
    getters: {
        isRoomOwner: state => state.id === state.owner,
        isOpponent: state => state.id === state.opponent,
        isPlayer() {
            return this.isRoomOwner || this.isOpponent;
        },
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
            this.started = message.started;
        },
        upsertUser(id: string, user: SocketUser) {
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
