<template>
    <TtInput
        v-model="username"
        name="username"
        :label="$t('roomJoiner.username')"
    />
</template>

<script lang="ts" setup>
import { provideValidators, validator } from '~/utils/Validator';
import { maxLength, notBlank } from '~/utils/StringValidator';
import { initUsernameAfterLoad, onMounted, watch } from '#imports';
import { useUsername } from '~/utils/UseUsername';

const username = useUsername();

onMounted(() => {
    initUsernameAfterLoad();
});

const emit = defineEmits<{ (e: 'update:isValid', value: boolean): void }>();

const nameValidator = validator(username, false, notBlank, maxLength(25));
provideValidators({ username: nameValidator });

const immediateValidator = validator(username, true, notBlank, maxLength(25));
watch(() => immediateValidator.isValid, newValue => emit('update:isValid', newValue == null || newValue), { immediate: true });
</script>
