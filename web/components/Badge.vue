<template>
    <span
        class="badge"
        :class="`theme-${props.theme}`"
    >
        <slot />
    </span>
</template>

<script lang="ts" setup>
const props = defineProps<{
    theme: 'warning' | 'info'
}>();
</script>

<style lang="scss" scoped>
.badge {
    border-radius: 4px;
    padding: 4px 8px;
    font-size: 1em;

    &.theme-warning {
        color: black;
        background-color: $warning-yellow;
    }

    &.theme-info {
        color: white;
        background-color: $info-blue;
    }
}
</style>
