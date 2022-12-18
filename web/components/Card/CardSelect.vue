<template>
    <div class="card-select">
        <Card
            v-for="card in cards"
            :key="`card_${card}`"
            :name="card"
            theme="details"
            :clickable="!disabled"
            :disabled="disabledItems.includes(card)"
            :active="props.modelValue === card"
            @click="onCardClick(card)"
        />
    </div>
</template>

<script lang="ts" setup>
import { CardMap } from '~/helpers/Cards';
import { countCardSquares } from '~/helpers/SquareHelper';

const emit = defineEmits<{
    (e: 'update:modelValue', value: string): void
}>();

const props = withDefaults(defineProps<{
    modelValue: string | null
    disabled?: boolean
    disabledItems?: string[]
}>(), {
    disabled: false,
    disabledItems: () => []
});

function onCardClick(card: string) {
    if (!props.disabled && !props.disabledItems.includes(card)) {
        emit('update:modelValue', card);
    }
}

const cards = Array.from(CardMap.values())
    .map(card => ({ name: card.name, squareCount: countCardSquares(card.squares), number: card.number }))
    .sort((a, b) => a.squareCount - b.squareCount || a.number - b.number)
    .map(card => card.name);
</script>

<style lang="scss" scoped>
.card-select {
    display: grid;
    grid-template-columns: repeat(5, 1fr);
    gap: 8px;
}
</style>
