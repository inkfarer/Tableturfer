<template>
    <div>
        <NuxtLayout>
            <NuxtPage />
        </NuxtLayout>
    </div>
</template>

<script lang="ts" setup>
import { useHead, useI18n, useProductionBuild } from '#imports';

const i18n = useI18n();
const isProd = useProductionBuild();

useHead({
    titleTemplate: chunk => {
        let result = chunk == null
            ? i18n.t('title.default')
            : i18n.t('title.template', { page: chunk });
        if (!isProd) {
            result = `${result} ${i18n.t('title.devBuildMarker')}`;
        }

        return result;
    }
});
</script>
