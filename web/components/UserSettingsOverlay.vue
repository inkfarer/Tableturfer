<template>
    <Overlay v-model="isOpen">
        <div class="settings-overlay">
            <h2>{{ $t('userSettings.title') }}</h2>
            <p>{{ $t('userSettings.subtitle') }}</p>
            <TtToggle
                v-model="userSettingsStore.useOnScreenMovementControls"
                :label="$t('userSettings.onScreenMovementControls')"
            />
            <TtToggle
                v-model="userSettingsStore.useOnScreenRotationAndPlacementControls"
                :label="$t('userSettings.onScreenRotationAndPlacementControls')"
                class="mt-1x"
            />
            <TtToggle
                :model-value="userSettingsStore.flipBoardOnBravoTeam"
                :label="$t('userSettings.flipBoardOnBravoTeam')"
                class="mt-1x"
                @update:model-value="userSettingsStore.setBoardFlip($event)"
            />
        </div>
    </Overlay>
</template>

<script lang="ts" setup>
import { ref } from '#imports';
import { useUserSettingsStore } from '~/stores/UserSettingsStore';

const isOpen = ref(false);
function open() {
    isOpen.value = true;
}

defineExpose({ open });

const userSettingsStore = useUserSettingsStore();
userSettingsStore.$subscribe(() => {
    userSettingsStore.save();
});
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
