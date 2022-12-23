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
import { watch } from '#imports';
import { useUsername } from '~/utils/UseUsername';

const username = useUsername();

// todo: remember the name used between visits
const nameValidator = validator(username, false, notBlank, maxLength(25));
provideValidators({ username: nameValidator });

const emit = defineEmits<{ (e: 'update:isValid', value: boolean): void }>();
watch(() => nameValidator.isValid, newValue => emit('update:isValid', newValue === true), { immediate: true });
</script>
