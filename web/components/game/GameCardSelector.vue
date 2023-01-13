<template>
    <div class="game-card-selector">
        <GameRedrawMessage />
        <GameGuideOverlay ref="guideOverlay" />
        <UserSettingsOverlay ref="userSettingsOverlay" />
        <Card
            :name="longPressedCard"
            :team="roomStore.playerTeam"
            theme="details"
            class="preview-card"
            :class="{ 'long-pressed': longPressActive }"
        />
        <div class="selectable-card-grid">
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
        </div>
        <div class="move-option-toggles mt-1x">
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
        <div class="secondary-action-buttons mt-1x">
            <TtButton
                theme="primary-small"
                @click="
                    // @ts-ignore
                    $refs.guideOverlay.open()
                "
            >
                <Icon name="fa6-solid:question" />
            </TtButton>
            <TtButton
                theme="primary-small"
                @click="
                    // @ts-ignore
                    $refs.userSettingsOverlay.open()
                "
            >
                <Icon name="fa6-solid:gear" />
            </TtButton>
        </div>
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
    width: 100%;
}

.selectable-card-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 8px;
}

.move-option-toggles {
    display: flex;

    > * {
        width: 100%;

        &:not(:first-child) {
            margin-left: 8px;
        }
    }
}

.secondary-action-buttons {
    display: flex;
    justify-content: center;

    > * {
        font-size: 1.2em !important;

        &:not(:first-child) {
            margin-left: 8px;
        }
    }
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
    .selectable-card-grid {
        grid-template-columns: unset;
        grid-auto-flow: column;
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
