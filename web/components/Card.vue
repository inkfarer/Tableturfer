<template>
    <div
        class="card-preview"
        :class="{
            [`theme-${props.theme}`]: true,
            'missing-card-data': cardData == null,
            active,
            clickable,
            disabled
        }"
        @click.prevent="handleClick"
    >
        <div
            v-if="cardData == null"
            class="card-placeholder"
            :class="{
                active,
                clickable
            }"
        >
            <slot name="placeholder" />
        </div>

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
                    :square="CardSquareType.SPECIAL"
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
        type: String as PropType<'card' | 'details' | 'miniature'>,
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
    position: absolute;
    height: 100%;
    width: 100%;
    background-color: rgba(0, 0, 0, 0.4);
    border-radius: 8px;
    border-style: solid;
    border-color: $accent;
    border-width: 0;
    box-sizing: border-box;
    display: flex;
    align-items: center;
    justify-content: center;
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
    position: relative;
    padding: 4px;
    border: 2px solid $accent;
    box-sizing: border-box;
    text-align: center;
    transition: background-color $default-transition-duration, filter $default-transition-duration;

    &:before {
        content: '';
        position: absolute;
        left: 0;
        top: 0;
        width: 100%;
        height: 100%;
        background-color: $page-background;
        z-index: -1;
    }

    &.missing-card-data {
        padding: 0;
        border-width: 0;
        background-color: transparent !important;

        .card-name, .card-square-preview, .cost {
            visibility: hidden;
        }
    }

    &.theme-details, &.theme-miniature {
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
        padding: 0.2em;
        align-self: stretch;

        .square-count {
            background-color: $accent;
            color: #222;
            padding: 0.15em 0.2em;
            font-size: 2em;
            font-weight: 700;
            min-width: 1em;
            text-align: center;
            border-radius: 8px;
        }

        .special-cost {
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(0, min(calc(100% - 0.2em * 2) / 3, 1.3em)));
            flex-grow: 1;
            gap: 0.2em;
            margin: 0.2em 0.2em 0.2em 0.3em;
        }
    }
}

@include media-breakpoint-down(md) {
    .card-preview.theme-miniature {
        > .cost {
            display: none;
        }
    }
}
</style>
