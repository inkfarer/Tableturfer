<template>
    <transition name="full-deck-view">
        <div
            v-if="props.isOpen"
            class="full-deck-view"
        >
            <div
                class="scrollable-content"
            >
                <Card
                    v-for="card in playerDeck?.cards"
                    :key="card"
                    theme="details"
                    :name="card"
                    :team="roomStore.playerTeam"
                    :disabled="usedCards?.has(card)"
                    :active="deckStore.availableCards.includes(card)"
                />
            </div>
            <div class="close-button-wrapper">
                <TtButton
                    theme="secondary"
                    @click="emit('update:isOpen', false)"
                >
                    <Icon
                        size="2em"
                        name="fa6-solid:xmark"
                    />
                </TtButton>
            </div>
        </div>
    </transition>
</template>

<script lang="ts" setup>
import { useDeckStore } from '~/stores/DeckStore';
import { computed } from '#imports';
import { useRoomStore } from '~/stores/RoomStore';

const props = defineProps<{
    isOpen: boolean
}>();

const emit = defineEmits<{
    (e: 'update:isOpen', value: boolean): void
}>();

const deckStore = useDeckStore();
const roomStore = useRoomStore();

const playerDeck = computed(() => deckStore.deck);
const usedCards = computed(() => roomStore.playerTeam == null ? null : deckStore.usedCards[roomStore.playerTeam]);
</script>

<style lang="scss" scoped>
.full-deck-view-enter-active {
    transition: opacity 250ms ease, transform 200ms ease-out;
}
.full-deck-view-leave-active {
    transition: opacity 250ms ease, transform 200ms ease-in;
}

.full-deck-view-enter-from,
.full-deck-view-leave-to {
    opacity: 0;
    transform: translateX(-25px);
}

.full-deck-view {
    overflow: hidden;
    display: flex;
    flex-direction: column;
    filter: drop-shadow(3px 3px 5px #000);
    pointer-events: initial;

    //&.open {
    //    opacity: 1;
    //    pointer-events: initial;
    //    transform: translate(0, 0);
    //}

    .scrollable-content {
        border: 2px solid $accent;
        background-color: #262626;
        padding: 8px;
        overflow-y: auto;
        display: grid;
        grid-template-columns: repeat(3, 1fr);
        gap: 4px;
        font-size: 0.7em;
    }
}

.close-button-wrapper {
    width: max-content;
    position: relative;
    left: 50%;
    top: -2px;
    padding-top: 2px;
    transform: translateX(-50%);
    background-color: #262626;
    border-width: 0 2px 2px 2px;
    border-style: solid;
    border-color: $accent;
    display: none;
}

@include media-breakpoint-down(lg) {
    .scrollable-content {
        grid-template-columns: repeat(auto-fit, minmax(max(calc(100% - 4px * 3) / 4, 7em), 1fr)) !important;
    }

    .close-button-wrapper {
        display: block;
    }

    .full-deck-view-enter-from,
    .full-deck-view-leave-to {
        opacity: 0;
        transform: translateY(25px);
    }
}
</style>
