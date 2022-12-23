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
            @click="emit('connect')"
        >
            {{ $t('room.noUsername.continue') }}
        </TtButton>
    </Overlay>
</template>

<script lang="ts" setup>
import { ref } from '#imports';

const usernameValid = ref(false);
const props = defineProps<{
    visible: boolean
}>();

const emit = defineEmits(['connect']);
</script>
