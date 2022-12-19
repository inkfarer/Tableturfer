import { defineNuxtPlugin } from '#imports';

export default defineNuxtPlugin(({ $pinia, $i18n }) => {
    $pinia.use(() => ({ $i18n }));
});
