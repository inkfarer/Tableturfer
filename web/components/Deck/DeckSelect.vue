<template>
    <div class="deck-select">
        <div
            v-for="deck in deckListStore.decks"
            :key="`deck_${deck.id}`"
            class="deck"
            :class="{
                selected: props.modelValue === deck.id
            }"
            @click="emit('update:modelValue', deck.id)"
        >
            {{ deck.name }}
        </div>
    </div>
</template>

<script lang="ts" setup>
import { useDeckListStore } from '~/stores/DeckListStore';

const emit = defineEmits<{
    (e: 'update:modelValue', value: string): void
}>();

const props = defineProps<{
    modelValue: string | null
}>();

const deckListStore = useDeckListStore();
</script>

<style lang="scss" scoped>
.deck-select {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 8px;

    > .deck {
        border: 2px solid $accent;
        background-color: $accent-a10;
        text-align: center;
        border-radius: 8px;
        padding: 12px 0;
        font-size: 1.25em;
        font-weight: 500;
        transition: background-color $default-transition-duration;
        cursor: pointer;
        user-select: none;

        &.selected {
            background-color: $accent-a35 !important;
        }

        &:hover {
            background-color: $accent-a20;
        }

        &:active {
            background-color: $accent-a35;
        }
    }
}
</style>
