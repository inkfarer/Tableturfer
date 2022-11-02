import { defineStore } from 'pinia';
import { PlayerTeam } from '~/types/PlayerTeam';

interface GameStateStore {
    playerTeam: PlayerTeam | null
}

export const useGameStateStore = defineStore('gameState', {
    state: (): GameStateStore => ({
        playerTeam: null
    })
});
