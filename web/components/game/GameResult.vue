<template>
    <div class="game-result">
        <div class="score-bar">
            <template v-if="roomStore.score != null">
                <div
                    class="team-score team-alpha"
                    :style="{ width: `${(roomStore.score.Alpha / totalScore) * 100}%` }"
                >
                    {{ roomStore.score.Alpha }}
                </div>
                <div
                    class="team-score team-bravo"
                    :style="{ width: `${(roomStore.score.Bravo / totalScore) * 100}%` }"
                >
                    {{ roomStore.score.Bravo }}
                </div>
            </template>
        </div>
        <div
            v-if="roomStore.score != null"
            class="result-text"
        >
            <template v-if="roomStore.score.Alpha === roomStore.score.Bravo">
                It's a draw!
            </template>
            <template v-else-if="roomStore.score.Alpha > roomStore.score.Bravo">
                {{ roomStore.ownerUser?.username ?? 'Alpha team' }} wins!
            </template>
            <template v-else>
                {{ roomStore.opponentUser?.username ?? 'Bravo team' }} wins!
            </template>
        </div>
        <TtButton
            v-if="roomStore.isRoomOwner"
            class="return-button"
            inline
            @click="returnToRoom"
        >
            {{ $t('game.returnToRoom') }}
        </TtButton>
        <div
            v-else
            class="mt-1x"
        >
            {{ $t('game.waitingToReturn') }}
        </div>
    </div>
</template>

<script lang="ts" setup>
import { useRoomStore } from '~/stores/RoomStore';
import { computed, useNuxtApp } from '#imports';

const roomStore = useRoomStore();
const { $socket } = useNuxtApp();

const totalScore = computed(() => roomStore.score == null ? 0 : roomStore.score.Alpha + roomStore.score.Bravo);

function returnToRoom() {
    $socket.send('ReturnToRoom');
}
</script>

<style lang="scss" scoped>
.game-result {
    width: 100%;
    text-align: center;
}

.score-bar {
    height: 45px;
    background-color: rgba(0, 0, 0, 0.25);
    border: 2px solid $accent;
    display: flex;

    .team-score {
        height: 100%;
        font-size: 1.75em;
        font-weight: 600;
        line-height: 45px;

        &.team-alpha {
            background-color: #E9FF0F;
            color: #222;
            text-align: left;
            padding-left: 8px;
        }

        &.team-bravo {
            background-color: #4B50F3;
            color: white;
            text-align: right;
            padding-right: 8px;
        }
    }
}

.return-button, .result-text {
    margin-top: 4px;
}
</style>
