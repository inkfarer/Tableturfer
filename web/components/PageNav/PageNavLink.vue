<template>
    <NuxtLink
        :to="props.to"
        active-class="active"
    >
        <slot />
    </NuxtLink>
</template>

<script lang="ts" setup>
const props = defineProps({
    to: {
        type: String,
        required: true
    }
});
</script>

<style lang="scss" scoped>
a {
    color: white;
    transition: color $default-transition-duration;
    text-decoration: none;
    font-size: 1.25em;

    &:hover {
        color: $accent;
    }

    &.active {
        color: $accent;

        &:hover {
            color: $accent-active;
        }
    }
}
</style>
