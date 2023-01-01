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
    background-size: 100%;

    &.team_Alpha {
        &.type_1 {
            background-image: url('/img/squares/1x/fill-alpha.webp');
        }

        &.type_2 {
            background-image: url('/img/squares/1x/special-alpha.webp');
        }
    }

    &.team_Bravo {
        &.type_1 {
            background-image: url('/img/squares/1x/fill-bravo.webp');
        }

        &.type_2 {
            background-image: url('/img/squares/1x/special-bravo.webp');
        }
    }
}
</style>
