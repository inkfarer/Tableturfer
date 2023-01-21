<template>
    <transition
        name="background-wrapper"
        appear
    >
        <div
            v-if="!roomStore.redrawCompleted"
            class="background-wrapper"
        >
            <div class="background" />
        </div>
    </transition>
    <transition
        name="redraw-message"
        appear
    >
        <OverlayBody
            v-if="!roomStore.redrawCompleted"
            class="redraw-message-body"
        >
            <div class="text">
                {{ $t('game.redrawMessage.title') }}
                <div class="mobile-card-preview-hint">
                    {{ $t('game.redrawMessage.mobileCardPreviewHint') }}
                </div>
            </div>
            <div class="buttons">
                <TtButton @click="decline">
                    {{ $t('game.redrawMessage.decline') }}
                </TtButton>
                <TtButton @click="confirm">
                    {{ $t('game.redrawMessage.confirm') }}
                </TtButton>
            </div>
        </OverlayBody>
    </transition>
</template>

<script lang="ts" setup>
import { useRoomStore } from '~/stores/RoomStore';
import { useNuxtApp } from '#imports';
import { useCurrentMoveStore } from '~/stores/CurrentMoveStore';

const currentMoveStore = useCurrentMoveStore();
const roomStore = useRoomStore();
const { $socket } = useNuxtApp();


function confirm() {
    $socket.send('RequestRedraw');
    currentMoveStore.setActiveCard(null);
    roomStore.redrawCompleted = true;
}
function decline() {
    roomStore.redrawCompleted = true;
}
</script>

<style lang="scss" scoped>
.background-wrapper-enter-active,
.background-wrapper-leave-active {
    transition: opacity 250ms ease;
}

.background-wrapper-enter-from,
.background-wrapper-leave-to {
    opacity: 0;
}

.redraw-message-enter-active {
    transition: opacity 250ms ease, transform 200ms ease-out;
}
.redraw-message-leave-active {
    transition: opacity 250ms ease, transform 200ms ease-in;
}

.redraw-message-enter-from,
.redraw-message-leave-to {
    opacity: 0;
    transform: translateY(-50%) translateX(-25px);
}

.background-wrapper {
    position: absolute;
    top: 0;
    right: 0;

    .background {
        position: relative;

        &:after {
            content: '';
            position: fixed;
            top: 0;
            width: 100%;
            height: 100%;
            background: linear-gradient(90deg, rgba(0, 0, 0, 0) 0%, rgba(0, 0, 0, 0.5) 50px, rgba(0, 0, 0, 0.75) 100%);
        }
    }
}

.redraw-message-body {
    width: 250px;
    max-width: 80vw;
    max-height: unset;

    > .text {
        text-align: center;
        font-size: 1.3em;
        margin-top: 4px;

        > .mobile-card-preview-hint {
            display: none;
            opacity: 0.75;
            font-size: 0.8em;
            margin-top: 4px;
        }
    }

    > .buttons {
        margin-top: 8px;
        display: grid;
        grid-template-columns: repeat(2, 1fr);
        gap: 8px;
    }
}

@include media-breakpoint-down(lg) {
    .redraw-message-enter-from,
    .redraw-message-leave-to {
        opacity: 0;
        transform: translateX(-50%) translateY(25px);
    }

    .background-wrapper .background:after {
        top: unset;
        left: 0;
        height: 100%;
        min-height: 1000px;
        transform-origin: top center;
        transform: rotate(180deg);
        background: linear-gradient(rgba(0, 0, 0, 0) 0%, rgba(0, 0, 0, 0.5) 100px, rgba(0, 0, 0, 0.75) 100%);
    }
}

@include media-breakpoint-down(md) {
    .redraw-message-body .mobile-card-preview-hint {
        display: block !important;
    }
}
</style>
