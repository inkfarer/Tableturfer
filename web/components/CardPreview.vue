<template>
    <div class="card-preview">
        <div class="card-name">
            {{ cardData == null ? '???' : $t(`game.card.${cardData.name}`) }}
        </div>
        <CardSquarePreview
            :squares="cardData?.squares ?? []"
            :team="props.team"
        />
        <div class="cost">
            <div class="square-count">
                {{ squareCount }}
            </div>
            <div class="special-cost">
                <CardSquare
                    v-for="point in cardData?.specialCost ?? 0"
                    :key="`special-cost-point_${point}`"
                    :team="props.team"
                    :square="CardSquareType.FILL"
                />
            </div>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { ComputedRef, PropType } from 'vue';
import { Card } from '~/types/Card';
import { computed } from '#imports';
import { CardMap } from '~/helpers/Cards';
import { count2D } from '~/helpers/ArrayHelper';
import { CardSquareType } from '~/types/CardSquareType';
import { PlayerTeam } from '~/types/PlayerTeam';

const props = defineProps({
    name: {
        type: String as PropType<string | null>,
        default: null
    },
    team: {
        type: String as PropType<PlayerTeam>,
        default: PlayerTeam.ALPHA
    }
});

const cardData: ComputedRef<Card | undefined> = computed(() => props.name == null ? undefined : CardMap.get(props.name));
const squareCount = computed(() => {
    if (cardData.value == null) {
        return 0;
    }

    return count2D(cardData.value?.squares, item => item !== CardSquareType.EMPTY);
});
</script>

<style lang="scss" scoped>
.card-preview {
    padding: 4px;
    border: 2px solid $accent;
    background-color: #1B1B1B;
    text-align: center;

    > .card-name {
        font-size: 1.25em;
        font-weight: 600;
        margin: 4px 0;
        background-color: rgba(0, 0, 0, 0.5);
        border-radius: 8px;
    }

    > .card-square-preview {
        margin: 0 4px;
        border-radius: 8px;
    }

    > .cost {
        display: flex;
        align-items: center;
        margin-top: 4px;
        background-color: rgba(0, 0, 0, 0.5);
        border-radius: 8px;
        padding: 4px;

        .square-count {
            background-color: $accent;
            color: #222;
            padding: 5px;
            font-size: 2em;
            font-weight: 700;
            min-width: 40px;
            text-align: center;
            border-radius: 8px;
        }

        .special-cost {
            width: 100%;
            display: grid;
            grid-template-columns: repeat(auto-fit, 20px);
            gap: 4px;
            margin-left: 4px;
        }
    }
}
</style>
