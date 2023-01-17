import { PlayerTeam, TeamMap } from '~/types/PlayerTeam';
import { PlayerMove } from '~/types/socket/SocketCommon';
import { defineStore } from 'pinia';
import { useCurrentMoveStore } from '~/stores/CurrentMoveStore';
import { useGameBoardStore } from '~/stores/GameBoardStore';
import { useDeckStore } from '~/stores/DeckStore';
import { useRoomStore } from '~/stores/RoomStore';

interface MoveStore {
    completedMoves: Array<TeamMap<PlayerMove>>
    nextMoveCompleted: TeamMap<boolean>
}

export const useMoveStore = defineStore('move', {
    state: (): MoveStore => ({
        completedMoves: [],
        nextMoveCompleted: {
            [PlayerTeam.ALPHA]: false,
            [PlayerTeam.BRAVO]: false
        }
    }),
    getters: {
        lastMove: state => state.completedMoves.length === 0 ? null : state.completedMoves[state.completedMoves.length - 1],
        passesForTeam: state => (team: PlayerTeam) => state.completedMoves.filter(move => move[team].type === 'Pass').length
    },
    actions: {
        applyMoves(moves: TeamMap<PlayerMove>) {
            this.nextMoveCompleted = {
                [PlayerTeam.ALPHA]: false,
                [PlayerTeam.BRAVO]: false
            };
            this.completedMoves.push(moves);

            useCurrentMoveStore().onNewMove();
            useGameBoardStore().applyMoves(moves);
            useDeckStore().setUsedCards(moves);
            useRoomStore().remainingTurns--;
        },
        resetMoves() {
            this.$reset();
        }
    }
});
