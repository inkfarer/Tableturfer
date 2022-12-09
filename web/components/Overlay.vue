<template>
    <div
        class="overlay-wrapper"
        :class="{
            open: props.modelValue,
            'hide-background': props.hideBackground
        }"
        @click.exact="close"
    >
        <div class="overlay">
            <slot />
        </div>
    </div>
</template>

<script lang="ts" setup>
const props = defineProps({
    modelValue: {
        type: Boolean,
        required: true
    },
    closable: {
        type: Boolean,
        default: true
    },
    hideBackground: {
        type: Boolean,
        default: false
    }
});

const emit = defineEmits(['update:modelValue']);

function close() {
    if (props.closable) {
        emit('update:modelValue', false);
    }
}
</script>

<style lang="scss" scoped>
.overlay-wrapper {
    position: absolute;
    z-index: 9999999;
    top: 0;
    left: 0;
    height: 100vh;
    width: 100vw;
    background-color: rgba(0, 0, 0, 0.5);
    display: none;
    opacity: 0;
    transition: opacity 250ms;
    flex-direction: column;
    justify-content: center;

    &.hide-background {
        background-color: rgba(18, 18, 18, 0.99);
    }

    &.open {
        opacity: 1;
        display: flex;
    }

    > .overlay {
        margin: 0 auto;
        padding: 10px;
        border-radius: 16px;
        background-color: #1B1B1B;
        border: 2px solid $accent;
        min-width: 200px;
    }
}
</style>
