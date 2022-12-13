<template>
    <div
        class="alert"
        :class="`theme-${props.theme}`"
    >
        <slot />
    </div>
</template>

<script lang="ts" setup>
const props = withDefaults(defineProps<{
    theme?: 'error' | 'info'
}>(), {
    theme: 'error'
});
</script>

<style lang="scss" scoped>
.alert {
    padding: 8px 12px;
    border-radius: 16px;
    border-style: solid;
    border-width: 2px;

    &.theme-error {
        border-color: $error-red;
        background-color: $error-red-a20;
    }

    &.theme-info {
        border-color: $info-blue;
        background-color: $info-blue-a20;
    }
}
</style>
