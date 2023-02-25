<template>
    <Overlay v-model="isOpen">
        <div
            v-if="config != null"
            class="settings-overlay"
        >
            <h2>{{ $t('roomConfig.title') }}</h2>
            <p>{{ $t('roomConfig.subtitle') }}</p>
            <TtToggle
                :model-value="config.turnTimerSeconds > 0"
                :label="$t('roomConfig.enableTurnTimer.label')"
                :details="$t('roomConfig.enableTurnTimer.details')"
                @update:model-value="config.turnTimerSeconds = $event ? enabledTurnTimerValue : 0"
            />
            <TtButton
                class="mt-1x"
                @click="saveChanges"
            >
                {{ $t('roomConfig.save') }}
            </TtButton>
        </div>
    </Overlay>
</template>

<script lang="ts" setup>
import { ref, useNuxtApp } from '#imports';
import { useRoomStore } from '~/stores/RoomStore';
import { RoomConfig } from '~/types/socket/SocketCommon';
import cloneDeep from 'lodash/cloneDeep';

const enabledTurnTimerValue = 60;
const config = ref<RoomConfig | null>(null);
const roomStore = useRoomStore();
const { $socket } = useNuxtApp();

const isOpen = ref(false);
function open() {
    if (roomStore.config == null) {
        console.warn('Cannot open room config, user is not in a room');
        return;
    }

    config.value = cloneDeep(roomStore.config);
    isOpen.value = true;
}

function saveChanges() {
    $socket.send('SetConfig', config.value);
    // todo: listen for config update event, then close?
    isOpen.value = false;
}

defineExpose({ open });
</script>

<style lang="scss" scoped>
.settings-overlay {
    text-align: center;
    max-width: 450px;
    margin: 0 35px 10px;
}

p {
    border-bottom: 2px solid $accent;
    padding-bottom: 15px;
    margin: 10px 0 10px;
}

@include media-breakpoint-down(md) {
    .settings-overlay {
        margin: 0 10px 10px;
    }
}
</style>
