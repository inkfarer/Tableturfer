<template>
    <label class="tt-input">
        {{ props.label }}
        <input
            v-model="model"
            type="text"
        >
    </label>
</template>

<script lang="ts" setup>
import { computed } from '#imports';

const props = withDefaults(defineProps<{
    modelValue: string | null
    label?: string
}>(), {
    label: ''
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
</script>

<style lang="scss" scoped>
label {
    display: block;
    color: rgba(255, 255, 255, 0.8);
    transition: color $default-transition-duration;

    &:focus-within {
        color: rgba(255, 255, 255, 1);
    }
}

input {
    background-color: rgba(17, 17, 17, 0.25);
    width: 100%;
    color: white;
    font-size: 1.4em;
    font-family: 'Roboto', sans-serif;
    display: block;
    box-sizing: border-box;
    margin: 2px 0;
    padding: 4px;
    border-radius: 8px;
    border: 2px solid $accent;
    transition: border-color $default-transition-duration;

    &:focus {
        outline: none;
        border-color: $accent-active;
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
