<template>
    <div>
        <slot />
    </div>
</template>

<script lang="ts" setup>
import { useHead } from '#imports';

useHead({
    bodyAttrs: {
        class: 'without-pull-to-refresh'
    }
});
</script>

<style lang="scss">
/* prevent pull-to-refresh for Safari 16+ */
@media screen and (pointer: coarse) {
    @supports (-webkit-backdrop-filter: blur(1px)) and (overscroll-behavior-y: none) {
        html:has(body.without-pull-to-refresh) {
            min-height: 100.3%;
            overscroll-behavior-y: none;
        }
    }
}
/* prevent pull-to-refresh for Safari 9~15 */
@media screen and (pointer: coarse) {
    @supports (-webkit-backdrop-filter: blur(1px)) and (not (overscroll-behavior-y: none)) {
        html:has(body.without-pull-to-refresh) {
            height: 100%;
            overflow: hidden;
        }
        body.without-pull-to-refresh {
            margin: 0px;
            max-height: 100%;
            overflow: auto;
            -webkit-overflow-scrolling: touch;
        }
    }
}

/* prevent pull-to-refresh for Chrome 63+ */
body.without-pull-to-refresh {
    overscroll-behavior-y: none;
}
</style>
