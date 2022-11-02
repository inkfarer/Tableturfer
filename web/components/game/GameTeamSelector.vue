<template>
    <div class="team-selector">
        <button @click="setTeam(PlayerTeam.ALPHA)">Alpha</button>
        <button @click="setTeam(PlayerTeam.BRAVO)">Bravo</button>
    </div>
</template>

<script lang="ts" setup>
import { PlayerTeam } from '~/types/PlayerTeam';
import { useGameStateStore } from '~/stores/GameStateStore';

const gameStateStore = useGameStateStore();
function setTeam(team: PlayerTeam) {
    gameStateStore.playerTeam = team;
}
</script>
