<template>
    <div
        class="card-preview"
        :class="{
            [`theme-${props.theme}`]: true,
            active,
            clickable
        }"
        @click.prevent="handleClick"
    >
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

const emit = defineEmits(['click']);

const props = defineProps({
    name: {
        type: String as PropType<string | null>,
        default: null
    },
    team: {
        type: String as PropType<PlayerTeam>,
        default: PlayerTeam.ALPHA
    },
    theme: {
        type: String as PropType<'card' | 'details'>,
        default: 'card'
    },
    active: {
        type: Boolean,
        default: false
    },
    clickable: {
        type: Boolean,
        default: false
    }
});

const cardData: ComputedRef<Card | undefined> = computed(() => props.name == null ? undefined : CardMap.get(props.name));
const squareCount = computed(() => {
    if (cardData.value == null) {
        return 0;
    }

    return count2D(cardData.value?.squares, item => item !== CardSquareType.EMPTY);
});

function handleClick() {
    if (props.clickable) {
        emit('click');
    }
}
</script>

<style lang="scss" scoped>
.card-preview {
    padding: 4px;
    border: 2px solid $accent;
    text-align: center;
    transition: background-color $default-transition-duration;

    &.theme-details {
        background-color: $accent-a10;

        &.active {
            background-color: $accent-a35;
        }

        &.clickable {
            cursor: pointer;

            &:hover {
                background-color: $accent-a50;
            }

            &:active {
                background-color: $accent-a75;
            }
        }
    }

    &.theme-card {
        background-color: #1B1B1B;
    }

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
