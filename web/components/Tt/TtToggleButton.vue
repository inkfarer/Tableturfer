<template>
    <a
        class="button"
        :class="{ disabled: props.disabled, active: modelValue }"
        href="javascript:void(0);"
        @click="toggle"
    >
        <slot />
    </a>
</template>

<script lang="ts" setup>
const props = defineProps({
    modelValue: {
        type: Boolean,
        required: true
    },
    disabled: {
        type: Boolean,
        default: false
    }
});

const emit = defineEmits(['update:modelValue']);

function toggle() {
    if (!props.disabled) {
        emit('update:modelValue', !props.modelValue);
    }
}
</script>

<style lang="scss" scoped>
.button {
    display: block;
    color: white;
    text-decoration: none;
    font-size: 1.5em;
    font-weight: 600;
    background-color: rgba(0, 0, 0, 0.4);
    padding: 10px 15px;
    transition: color $default-transition-duration, background-color $default-transition-duration;
    text-align: center;
    cursor: pointer;
    user-select: none;

    &.active {
        background-color: $accent;
        color: #222;
    }

    &:not(.disabled) {
        &:hover {
            background-color: $accent;
            color: #222;
        }

        &:active {
            background-color: $accent-active;
            color: #222;
        }

        &.active {
            &:hover {
                background-color: $accent-hover;
            }

            &:active {
                background-color: $accent-active;
            }
        }
    }

    &.disabled {
        filter: brightness(0.75);
        cursor: initial;
    }
}
</style>
