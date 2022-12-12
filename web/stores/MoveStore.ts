import { PlayerTeam, TeamMap } from '~/types/PlayerTeam';
import { PlayerMove } from '~/types/socket/SocketCommon';
import { defineStore } from 'pinia';
import { useCurrentMoveStore } from '~/stores/CurrentMoveStore';
import { useGameBoardStore } from '~/stores/GameBoardStore';
import { useDeckStore } from '~/stores/DeckStore';
import { useRoomStore } from '~/stores/RoomStore';

interface MoveStore {
    lastMoves: TeamMap<PlayerMove | null>
    nextMoveCompleted: TeamMap<boolean>
}

export const useMoveStore = defineStore('move', {
    state: (): MoveStore => ({
        lastMoves: {
            [PlayerTeam.ALPHA]: null,
            [PlayerTeam.BRAVO]: null
        },
        nextMoveCompleted: {
            [PlayerTeam.ALPHA]: false,
            [PlayerTeam.BRAVO]: false
        }
    }),
    actions: {
        applyMoves(moves: TeamMap<PlayerMove>) {
            this.nextMoveCompleted = {
                [PlayerTeam.ALPHA]: false,
                [PlayerTeam.BRAVO]: false
            };
            this.lastMoves = moves;

            useCurrentMoveStore().onNewMove();
            useGameBoardStore().applyMoves(moves);
            useDeckStore().setUsedCards(moves);
            useRoomStore().remainingTurns--;
        }
    }
});
