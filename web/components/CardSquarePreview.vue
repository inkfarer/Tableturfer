<template>
    <div class="card-square-preview">
        <div
            class="card-squares"
            :style="{
                transform: `translate(${offset.x}, ${offset.y})`
            }"
        >
            <div
                v-for="(row, y) in props.squares"
                :key="`square-row_${y}`"
                class="square-row"
                :style="{
                    width: `${size.width / Constants.CARD_GRID_SIZE * 100}%`
                }"
            >
                <CardSquare
                    v-for="(square, x) in row"
                    :key="`square_${x}_${y}`"
                    :square="square"
                    :team="props.team"
                />
            </div>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { CardSquareType } from '~/types/CardSquareType';
import { PropType } from 'vue';
import Constants from '~/data/Constants';
import { computed } from '#imports';
import { getSize } from '~/helpers/ArrayHelper';
import CardSquare from '~/components/CardSquare.vue';
import { PlayerTeam } from '~/types/PlayerTeam';

const props = defineProps({
    squares: {
        type: Array as PropType<Array<Array<CardSquareType>>>,
        required: true
    },
    team: {
        type: String as PropType<PlayerTeam>,
        required: true
    }
});

const size = computed(() => getSize(props.squares));
const offset = computed(() => {
    const offsetPosition = {
        x: Math.floor((Constants.CARD_GRID_SIZE - size.value.width) / 2),
        y: Math.floor((Constants.CARD_GRID_SIZE - size.value.height) / 2)
    };

    return {
        x: `${(offsetPosition.x / Constants.CARD_GRID_SIZE) * 100}%`,
        y: `${(offsetPosition.y / Constants.CARD_GRID_SIZE) * 100}%`
    };
});
</script>

<style lang="scss" scoped>
.card-square-preview {
    aspect-ratio: 1 / 1;
    background-size: 12.5% 12.5%;
    background-position: -1px -1px;
    overflow: hidden;
    background-color: #1B1B1B;
    background-image:
        linear-gradient(to right, #262626 2px, transparent 2px),
        linear-gradient(to bottom, #262626 2px, transparent 2px);

    .card-squares {
        transform-origin: top left;
        width: 100%;
        height: 100%;
    }

    .square-row {
        display: flex;
    }
}
</style>
