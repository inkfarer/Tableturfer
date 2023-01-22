<template>
    <Overlay
        :model-value="props.visible"
        hide-background
        :closable="false"
    >
        <p class="text-center">{{ $t('room.noUsername.title') }}</p>
        <PlayerNameInput
            @update:is-valid="usernameValid = $event"
        />
        <TtButton
            :disabled="!usernameValid"
            class="mt-1x"
            @click="onContinue"
        >
            {{ $t('room.noUsername.continue') }}
        </TtButton>
    </Overlay>
</template>

<script lang="ts" setup>
import { ref } from '#imports';
import { saveUsername } from '~/utils/UseUsername';

const usernameValid = ref(false);
const props = defineProps<{
    visible: boolean
}>();

const emit = defineEmits(['connect']);

function onContinue() {
    saveUsername();
    emit('connect');
}
</script>
