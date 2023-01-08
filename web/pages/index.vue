<template>
    <NuxtLayout name="default">
        <template #header>
            <div class="width-cap header-extra">
                <h1>{{ $t('index.title') }}</h1>
                <h2>{{ $t('index.subtitle') }}</h2>
                <Badge
                    theme="warning"
                    class="wip-badge"
                >
                    {{ $t('index.wipWarning') }}
                </Badge>
                <RoomJoiner />
            </div>
        </template>
    </NuxtLayout>
</template>

<script lang="ts" setup>
import { definePageMeta } from '#imports';

definePageMeta({
    layout: false
});
</script>

<style lang="scss" scoped>
.header-extra {
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
    min-height: 50vh;

    .wip-badge {
        text-align: center;
        align-self: center;
        margin-bottom: 30px;
    }

    h1 {
        font-weight: 800;
        font-size: 3em;
        margin: 0;
    }

    h2 {
        font-weight: 500;
        font-size: 2em;
        margin-top: 10px;
        margin-bottom: 5px;
    }

    .room-joiner-wrapper {
        margin: 0 45px 25px;
    }
}

@include media-breakpoint-down(md) {
    .header-extra {
        .room-joiner-wrapper {
            width: initial;
            margin: 0 auto 16px;
        }
    }
}
</style>
