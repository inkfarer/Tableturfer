<template>
    <div class="game-card-selector">
        <GameRedrawMessage />
        <Card
            :name="longPressedCard"
            :team="roomStore.playerTeam"
            theme="details"
            class="preview-card"
            :class="{ 'long-pressed': longPressActive }"
        />
        <Card
            v-for="card in deckStore.availableCards"
            :key="`card_${card}`"
            v-long-press
            :name="card"
            :active="activeCardStore.activeCard?.name === card"
            :clickable="!activeCardStore.locked"
            :team="roomStore.playerTeam"
            theme="miniature"
            class="card"
            @short-press="selectCard(card)"
            @long-press-start="onCardLongPress(card)"
            @long-press-stop="longPressActive = false"
        />
        <TtToggleButton
            :model-value="activeCardStore.pass"
            :disabled="activeCardStore.locked || !roomStore.redrawCompleted"
            class="action-button"
            @update:model-value="activeCardStore.setPass($event)"
        >
            {{ $t('game.pass') }}
        </TtToggleButton>
        <TtToggleButton
            :model-value="activeCardStore.special"
            :disabled="activeCardStore.locked || !roomStore.redrawCompleted"
            class="action-button"
            @update:model-value="activeCardStore.setSpecial($event)"
        >
            {{ $t('game.special') }}
        </TtToggleButton>
    </div>
</template>

<script lang="ts" setup>
import { CardMap } from '~/helpers/Cards';
import { useCurrentMoveStore } from '~/stores/CurrentMoveStore';
import { useDeckStore } from '~/stores/DeckStore';
import { ref, useNuxtApp } from '#imports';
import { useRoomStore } from '~/stores/RoomStore';

const roomStore = useRoomStore();
const activeCardStore = useCurrentMoveStore();
const deckStore = useDeckStore();
const { $socket } = useNuxtApp();
const longPressedCard = ref<string | null>(null);
const longPressActive = ref<boolean>(false);

const selectCard = (card: string) => {
    if (activeCardStore.pass) {
        activeCardStore.setActiveCard(CardMap.get(card) ?? null);
        $socket.send('ProposeMove', {
            type: 'Pass',
            cardName: card
        });
        activeCardStore.locked = true;
    } else {
        if (card != null && activeCardStore.activeCard?.name === card) {
            activeCardStore.setActiveCard(null);
        } else {
            activeCardStore.setActiveCard(CardMap.get(card) ?? null);
        }
    }
};

function onCardLongPress(card: string) {
    longPressedCard.value = card;
    longPressActive.value = true;
}
</script>

<style lang="scss">
.game-card-selector {
    position: relative;
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 8px;
}

.preview-card {
    display: none;
}

.redraw-message-body {
    position: absolute;
    left: 110%;
    top: 50%;
    transform: translateY(-50%);
}

@include media-breakpoint-down(lg) {
    .game-card-selector {
        grid-template-columns: repeat(4, 1fr);
    }

    .action-button {
        grid-column: span 2;
        height: max-content;
    }

    .redraw-message-body {
        bottom: 120%;
        top: unset;
        left: 50%;
        transform: translateX(-50%);
    }
}

@include media-breakpoint-down(md) {
    .preview-card {
        display: block;
        position: absolute !important;
        bottom: 120%;
        min-height: 100%;
        min-width: 160px;
        width: 50%;
        left: 50%;
        transform: translateX(-50%);
        opacity: 0;
        transition: opacity $default-transition-duration !important;
        pointer-events: none;

        &.long-pressed {
            opacity: 1;
        }
    }
}

@include media-breakpoint-down(sm) {
    .action-button {
        font-size: 1em !important;
    }
}
</style>
