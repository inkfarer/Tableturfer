<template>
    <div
        class="special-count"
        :class="`team_${props.team}`"
    >
        <CardSquare
            v-for="point in availableSpecial"
            :key="`available-point_${point}`"
            :square="CardSquareType.SPECIAL"
            :team="props.team"
        />
        <CardSquare
            v-for="point in gameBoardStore.usedSpecialPoints[props.team]"
            :key="`used-point_${point}`"
            :square="CardSquareType.SPECIAL"
            :team="props.team"
            class="used-point"
        />
        <CardSquare
            v-for="point in visibleBlankSquares"
            :key="`blank-point_${point}`"
            :square="CardSquareType.EMPTY"
            :team="props.team"
            class="blank-square"
        />
    </div>
</template>

<script lang="ts" setup>
import { PropType } from 'vue';
import { PlayerTeam } from '~/types/PlayerTeam';
import { useGameBoardStore } from '~/stores/GameBoardStore';
import { computed } from '#imports';
import { CardSquareType } from '~/types/CardSquareType';

const props = defineProps({
    team: {
        type: String as PropType<PlayerTeam>,
        required: true
    }
});
const gameBoardStore = useGameBoardStore();
const availableSpecial = computed(() => gameBoardStore.specialPointCount[props.team] - gameBoardStore.usedSpecialPoints[props.team]);
const visibleBlankSquares = computed(() => Math.max(0, 3 - gameBoardStore.specialPointCount[props.team]));
</script>

<style lang="scss" scoped>
.special-count {
    display: flex;
    min-height: 20px;
    flex-wrap: wrap;

    &.team_Bravo {
        flex-direction: row-reverse;
    }

    .square {
        width: 20px;
        margin: 2px;
    }

    .used-point {
        filter: brightness(0.5);
    }

    .blank-square {
        background-color: #000;
        opacity: 0.75;

        &:nth-child(2) {
            opacity: 0.5;
        }

        &:nth-child(3) {
            opacity: 0.25;
        }
    }
}
</style>
