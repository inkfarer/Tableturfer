<template>
    <Overlay v-model="isOpen">
        <div class="guide-overlay">
            <h2>{{ $t('howToPlay.mainHeading') }}</h2>
            <p>
                <i18n-t
                    keypath="howToPlay.intro"
                    scope="global"
                >
                    <template #wiki>
                        <NuxtLink href="https://splatoonwiki.org/wiki/Tableturf_Battle">{{ $t('howToPlay.wikiLinkText') }}</NuxtLink>
                    </template>
                </i18n-t>
            </p>
            <h3>{{ $t('howToPlay.controls.heading') }}</h3>
            <p>{{ $t('howToPlay.controls.mouseDescription') }}</p>
            <p>{{ $t('howToPlay.controls.touchDescription') }}</p>
            <TtButton @click="isOpen = false">{{ $t('howToPlay.dismiss') }}</TtButton>
        </div>
    </Overlay>
</template>

<script lang="ts" setup>
import { ref } from '#imports';

const isOpen = ref(false);
function open() {
    isOpen.value = true;
}

defineExpose({ open });
</script>

<style lang="scss" scoped>
.guide-overlay {
    text-align: center;
    max-width: 500px;
}

h3 {
    font-size: 1.3em;
}
</style>
