<template>
    <div>
        <NuxtLayout>
            <NuxtPage />
        </NuxtLayout>
    </div>
</template>

<script lang="ts" setup>
import { useHead, useI18n } from '#imports';

const i18n = useI18n();

useHead({
    titleTemplate: chunk => chunk == null ? i18n.t('title.default') : i18n.t('title.template', { page: chunk })
});
</script>
