<template>
    <a
        class="button"
        :class="{ disabled: props.disabled, inline: props.inline }"
        href="javascript:void(0);"
        @click="onClick"
    >
        <slot />
    </a>
</template>

<script lang="ts" setup>
const emit = defineEmits(['click']);

const props = defineProps({
    disabled: {
        type: Boolean,
        default: false
    },
    inline: {
        type: Boolean,
        default: false
    }
});

function onClick() {
    if (!props.disabled) {
        emit('click');
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

    &:not(.disabled) {
        &:hover {
            background-color: $accent;
            color: #222;
        }

        &:active {
            background-color: $accent-active;
            color: #222;
        }
    }

    &.disabled {
        color: #999;
        cursor: initial;
    }

    &.inline {
        display: inline-block;
    }
}
</style>
