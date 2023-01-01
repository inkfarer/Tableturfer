import { defineStore } from 'pinia';
import { SocketMessageMap, SocketUser } from '~/types/socket/SocketEvent';
import { PlayerTeam, TeamMap } from '~/types/PlayerTeam';
import { TURN_COUNT } from '~/data/Constants';

interface RoomStore {
    id: string | null
    roomCode: string | null
    owner: string | null
    opponent: string | null
    users: Record<string, SocketUser>
    started: boolean
    completed: boolean
    remainingTurns: number
    score: TeamMap<number> | null
    redrawCompleted: boolean
}

export const useRoomStore = defineStore('room', {
    state: (): RoomStore => ({
        id: null,
        roomCode: null,
        owner: null,
        opponent: null,
        users: {},
        started: false,
        completed: false,
        remainingTurns: TURN_COUNT,
        score: null,
        redrawCompleted: false
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
        },
        ownerUser: (state): SocketUser | null => {
            if (state.users == null || state.owner == null) {
                return null;
            }

            return state.users[state.owner] ?? null;
        },
        opponentUser: (state): SocketUser | null => {
            if (state.users == null || state.opponent == null) {
                return null;
            }

            return state.users[state.opponent] ?? null;
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
        resetGame() {
            this.started = false;
            this.completed = false;
            this.score = null;
            this.remainingTurns = TURN_COUNT;
        },
        leaveRoom() {
            this.$reset();
        }
    }
});
