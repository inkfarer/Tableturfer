<template>
    <Overlay v-model="isOpen">
        <div class="settings-overlay">
            <h2>{{ $t('userSettings.title') }}</h2>
            <p>{{ $t('userSettings.subtitle') }}</p>
            <TtToggle
                v-model="userSettingsStore.useOnScreenControls"
                :label="$t('userSettings.onScreenControls')"
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
    width: 400px;
    max-width: calc(85vw - 70px);
    margin: 0 35px 10px;
}

h2 {
    margin: 10px 0 0 0;
}

p {
    border-bottom: 2px solid $accent;
    padding-bottom: 15px;
    margin: 10px 0 10px;
}
</style>
