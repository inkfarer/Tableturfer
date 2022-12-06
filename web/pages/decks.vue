<template>
    <NuxtLayout name="default">
        <template #header>
            <div class="width-cap">
                <h1>Decks</h1>
            </div>
        </template>
        <div class="width-cap">
            <p>
                Custom decks are not currently implemented.
            </p>
        </div>
    </NuxtLayout>
</template>

<script lang="ts" setup>
import { definePageMeta } from '#imports';

definePageMeta({
    layout: false
});
</script>
