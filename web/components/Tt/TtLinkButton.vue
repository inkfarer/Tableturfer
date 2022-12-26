<template>
    <NuxtLink
        v-slot="{ href, navigate }"
        v-bind="props"
        custom
    >
        <TtButton
            v-bind="$attrs"
            :href="href"
            @click="navigate"
        >
            <slot />
        </TtButton>
    </NuxtLink>
</template>

<script lang="ts">
export default {
    inheritAttrs: false
};
</script>

<script lang="ts" setup>
const props = defineProps<{
    to: string
}>();
</script>
