<template>
    <div
        class="tt-toggle__wrapper"
        :class="{ disabled: disabled, active: modelValue }"
        @click="toggleValue"
    >
        {{ label }}
        <div class="value-display">
            <div class="indicator" />
        </div>
    </div>
</template>

<script lang="ts" setup>
const props = withDefaults(defineProps<{
    label: string
    modelValue?: boolean
    disabled?: boolean
}>(), {
    modelValue: false,
    disabled: false
});

const emit = defineEmits<{ (e: 'update:modelValue', value: boolean): void }>();

function toggleValue() {
    if (!props.disabled) {
        emit('update:modelValue', !props.modelValue);
    }
}
</script>

<style lang="scss" scoped>
.tt-toggle__wrapper {
    display: flex;
    align-items: center;
    justify-content: space-between;
    user-select: none;
    overflow-wrap: anywhere;
    text-align: left;

    &:not(.disabled) {
        cursor: pointer;

        &.active > .value-display {
            background-color: $accent;
        }

        &.active:hover > .value-display {
            background-color: $accent-hover;
        }

        &.active:active > .value-display {
            background-color: $accent-active;
        }

        &:hover > .value-display {
            background-color: $element-bg-neutral-hover;
        }

        &:active > .value-display {
            background-color: $element-bg-neutral-active;
        }
    }

    &.disabled {
        filter: brightness(0.75) contrast(0.9);
    }

    &.active > .value-display > .indicator {
        transform: translateX(100%);
        background-color: #222;
    }

    > .value-display {
        min-width: 50px;
        height: 28px;
        background-color: $element-bg-neutral;
        transition-duration: 250ms;
        margin-left: 8px;
        border-radius: 5px;

        > .indicator {
            height: 22px;
            width: 22px;
            background-color: #eee;
            margin: 3px;
            border-radius: 3px;
            transition-duration: 250ms;
            filter: drop-shadow(0 0 1px rgba(22, 22, 22, 0.5));
        }
    }
}
</style>
