<template>
    <div
        class="tt-input-wrapper"
        :class="{ 'has-error': !isValid }"
    >
        <label :for="`input_${uuid}`">
            {{ props.label }}
        </label>
        <div class="text-input-wrapper">
            <input
                :id="`input_${uuid}`"
                v-model="model"
                type="text"
            >
            <Icon
                v-if="!!validator"
                v-show="!isValid"
                class="error-icon"
                name="fa6-solid:circle-exclamation"
            />
        </div>
    </div>
</template>

<script lang="ts" setup>
import { computed, inject } from '#imports';
import { ValidatorMap, VALIDATOR_INJECTION_KEY } from '~/utils/Validator';
import { v4 as uuidv4 } from 'uuid';

const props = withDefaults(defineProps<{
    modelValue?: string | null
    label?: string
    name?: string | null
}>(), {
    modelValue: null,
    label: '',
    name: null
});

const emit = defineEmits<{
    (e: 'update:modelValue', value: string | null): void
}>();

const model = computed({
    get() {
        return props.modelValue ?? '';
    },
    set(value: string) {
        emit('update:modelValue', value);
    }
});

const uuid = uuidv4();

const validators = inject<ValidatorMap | null>(VALIDATOR_INJECTION_KEY, null);
const validator = computed(() => props.name == null || validators == null ? null : validators[props.name]);
const isValid = computed(() => !validator.value ? true : validator.value?.isValid ?? true);
</script>

<style lang="scss" scoped>
.tt-input-wrapper {
    text-align: left;

    &:focus-within {
        label {
            color: rgba(255, 255, 255, 1);
        }
    }

    &.has-error .text-input-wrapper {
        border-color: $error-red;
    }
}

label {
    display: block;
    color: rgba(255, 255, 255, 0.8);
    transition: color $default-transition-duration;
}

.text-input-wrapper {
    background-color: rgba(17, 17, 17, 0.25);
    color: white;
    padding: 4px;
    border-radius: 8px;
    border: 2px solid $accent;
    transition: border-color $default-transition-duration;
    margin: 2px 0;
    position: relative;

    &:focus-within {
        border-color: $accent-active;
    }

    > .error-icon {
        color: $error-red;
        position: absolute;
        right: 0.5em;
        bottom: 0.25em;
        font-size: 1.5em;
    }
}

input {
    width: 100%;
    font-size: 1.4em;
    font-family: 'Roboto', sans-serif;
    display: block;
    box-sizing: border-box;
    background-color: transparent;
    border: none;
    color: inherit;

    &:focus {
        outline: none;
    }

    &[type='number'] {
        -moz-appearance: textfield;
    }

    &[type='number']::-webkit-outer-spin-button,
    &[type='number']::-webkit-inner-spin-button {
        -webkit-appearance: none;
        margin: 0;
    }
}
</style>
