<template>
    <div
        class="overlay-wrapper"
        :class="{ open: props.modelValue }"
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
    }
});

const emit = defineEmits(['update:modelValue']);

function close() {
    emit('update:modelValue', false);
}
</script>

<style lang="scss" scoped>
.overlay-wrapper {
    position: absolute;
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
