<template>
    <div
        v-if="cardData != null"
        class="card-preview"
        :class="{
            [`theme-${props.theme}`]: true,
            active,
            clickable,
            disabled
        }"
        @click.prevent="handleClick"
    >
        <div
            v-if="props.theme === 'card'"
            class="card-name"
        >
            <span>{{ cardData == null ? '???' : $t(`game.card.${cardData.name}`) }}</span>
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
    <div
        v-else
        class="card-placeholder"
        :class="{
            active,
            clickable
        }"
        @click.prevent="handleClick"
    >
        <slot name="placeholder" />
    </div>
</template>

<script lang="ts" setup>
import { ComputedRef, PropType } from 'vue';
import { Card } from '~/types/Card';
import { computed } from '#imports';
import { CardMap } from '~/helpers/Cards';
import { CardSquareType } from '~/types/CardSquareType';
import { PlayerTeam } from '~/types/PlayerTeam';
import { countCardSquares } from '~/helpers/SquareHelper';

const emit = defineEmits(['click']);

const props = defineProps({
    name: {
        type: String as PropType<string | null>,
        default: null
    },
    team: {
        type: String as PropType<PlayerTeam | null>,
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
    },
    disabled: {
        type: Boolean,
        default: false
    }
});

const cardData: ComputedRef<Card | undefined> = computed(() => props.name == null ? undefined : CardMap.get(props.name));
const squareCount = computed(() => {
    if (cardData.value == null) {
        return 0;
    }

    return countCardSquares(cardData.value?.squares);
});

function handleClick() {
    if (props.clickable && !props.disabled) {
        emit('click');
    }
}
</script>

<style lang="scss" scoped>
.card-placeholder {
    background-color: rgba(0, 0, 0, 0.4);
    border-radius: 8px;
    border-style: solid;
    border-color: $accent;
    border-width: 0;
    box-sizing: border-box;
    transition:
        background-color $default-transition-duration,
        border-radius 250ms,
        border-width $default-transition-duration;

    &.active {
        background-color: rgba(15, 15, 15, 0.35);
        border-radius: 0;
        border-width: 2px;
    }

    &.clickable {
        cursor: pointer;

        &:hover {
            background-color: rgba(15, 15, 15, 0.4);
        }

        &:active {
            background-color: rgba(15, 15, 15, 0.6);
        }
    }
}

.card-preview {
    padding: 4px;
    border: 2px solid $accent;
    box-sizing: border-box;
    text-align: center;
    transition: background-color $default-transition-duration, filter $default-transition-duration;

    &.theme-details {
        background-color: $accent-a10;

        &.active {
            background-color: $accent-a35;
        }

        &.clickable:not(.disabled) {
            cursor: pointer;

            &:hover {
                background-color: $accent-a50;
            }

            &:active {
                background-color: $accent-a75;
            }
        }

        &.disabled {
            filter: brightness(0.5);
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
        min-height: 2.3em;
        display: flex;
        flex-direction: column;
        justify-content: center;
    }

    > .card-square-preview {
        border-radius: 8px;
    }

    > .cost {
        display: flex;
        align-items: center;
        margin-top: 4px;
        background-color: rgba(0, 0, 0, 0.5);
        border-radius: 8px;
        padding: 4px;
        align-self: stretch;

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
