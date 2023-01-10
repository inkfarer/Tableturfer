<template>
    <a
        class="button"
        :class="{
            disabled: props.disabled,
            inline: props.inline,
            [`theme-${props.theme}`]: true
        }"
        :href="props.href"
        @click="onClick"
    >
        <slot />
    </a>
</template>

<script lang="ts" setup>
const emit = defineEmits(['click']);

interface TtButtonProps {
    disabled?: boolean
    inline?: boolean
    href?: string
    theme?: 'primary' | 'secondary'
}

const props = withDefaults(defineProps<TtButtonProps>(), {
    disabled: false,
    inline: false,
    href: 'javascript:void(0);',
    theme: 'primary'
});

function onClick(event: MouseEvent) {
    if (!props.disabled) {
        emit('click', event);
    }
}
</script>

<style lang="scss">
.button > .icon {
    transform: translateY(-2px);
}
</style>

<style lang="scss" scoped>
.button {
    display: block;
    text-decoration: none;
    transition: color $default-transition-duration, background-color $default-transition-duration;
    text-align: center;
    cursor: pointer;
    user-select: none;

    &.theme-primary {
        font-size: 1.5em;
        font-weight: 600;
        background-color: rgba(0, 0, 0, 0.4);
        color: white;
        padding: 10px 15px;

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
    }

    &.theme-secondary {
        font-size: 1.1em;
        font-weight: 400;
        color: $accent;
        padding: 8px 12px;
        border-radius: 8px;

        &:not(.disabled) {
            &:hover {
                background-color: $accent-a10;
            }

            &:active {
                background-color: $accent-a20;
            }
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
