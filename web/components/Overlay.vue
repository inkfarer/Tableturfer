<template>
    <div
        class="overlay-wrapper"
        :class="{
            open: props.modelValue,
            'hide-background': props.hideBackground,
            'bottom-sheet': props.bottomSheet,
            'mobile-only': props.mobileOnly
        }"
        @click.self="close"
    >
        <OverlayBody class="overlay">
            <slot />
        </OverlayBody>
    </div>
</template>

<script lang="ts" setup>
const props = withDefaults(defineProps<{
    modelValue: boolean
    closable?: boolean
    hideBackground?: boolean
    bottomSheet?: boolean
    mobileOnly?: boolean
}>(), {
    closable: true,
    hideBackground: false,
    bottomSheet: false,
    mobileOnly: false
});

const emit = defineEmits<{
    (e: 'update:modelValue', value: boolean): void
}>();

function close() {
    if (props.closable) {
        emit('update:modelValue', false);
    }
}
</script>

<style lang="scss" scoped>
@mixin overlay-wrapper-styles {
    position: fixed;
    z-index: 9999999;
    bottom: 0;
    left: 0;
    height: 100%;
    width: 100%;
    opacity: 0;
    pointer-events: none;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
}

.overlay-wrapper:not(.mobile-only) {
    @include overlay-wrapper-styles;
}

.overlay-wrapper {
    display: contents;
    background-color: rgba(0, 0, 0, 0.5);
    transition: opacity $default-transition-duration;

    &.hide-background {
        background-color: rgba(18, 18, 18, 0.99);
    }

    &.open {
        opacity: 1;
        pointer-events: auto;
    }

    &.mobile-only > .overlay {
        display: contents;
    }
}

@include media-breakpoint-down(md) {
    .overlay-wrapper {
        @include overlay-wrapper-styles;

        &.bottom-sheet {
            &.open > .overlay {
                transform: translateY(0);
            }

            > .overlay {
                display: initial;
                border-bottom: 0;
                transition: transform 250ms;
                position: absolute;
                bottom: 0;
                width: calc(100% - 28px);
                max-width: 400px;
                border-radius: 16px 16px 0 0;
                padding: 12px 8px;
                transform: translateY(25px);
                background-color: #262626;
                margin-top: 50%;
            }
        }
    }
}
</style>
