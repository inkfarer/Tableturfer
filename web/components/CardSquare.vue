<template>
    <div
        class="square"
        :class="[`type_${props.square}`, `team_${props.team}`]"
    />
</template>

<script lang="ts" setup>
import { PropType } from 'vue';
import { CardSquareType } from '~/types/CardSquareType';
import { PlayerTeam } from '~/types/PlayerTeam';

const props = defineProps({
    square: {
        type: Number as PropType<CardSquareType>,
        required: true
    },
    team: {
        type: String as PropType<PlayerTeam | null>,
        default: PlayerTeam.ALPHA
    }
});
</script>

<style lang="scss" scoped>
.square {
    aspect-ratio: 1 / 1;
    width: 100%;

    &.team_Alpha {
        &.type_1 {
            background-color: #E9FF0F;
        }

        &.type_2 {
            background-color: rgba(236, 144, 9, 1);
        }
    }

    &.team_Bravo {
        &.type_1 {
            background-color: rgba(75, 80, 243, 1);
        }

        &.type_2 {
            background-color: rgba(21, 227, 219, 1);
        }
    }
}
</style>
