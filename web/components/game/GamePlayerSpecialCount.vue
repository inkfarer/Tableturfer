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
</script>

<style lang="scss" scoped>
.special-count {
    display: flex;
    min-height: 20px;

    &.team_Bravo {
        flex-direction: row-reverse;
    }

    .square {
        width: 20px;
        margin: 0 2px;
    }

    .used-point {
        filter: brightness(0.5);
    }
}
</style>
