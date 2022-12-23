<template>
    <NuxtLayout name="default">
        <template #header>
            <div class="width-cap header-extra">
                <h1>{{ $t('index.title') }}</h1>
                <h2>{{ $t('index.subtitle') }}</h2>
                <Alert>
                    {{ $t('index.smallScreen') }}
                </Alert>
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

    h1 {
        font-weight: 800;
        font-size: 3em;
        margin: 0;
    }

    h2 {
        font-weight: 500;
        font-size: 2em;
        margin-top: 10px;
    }

    .alert {
        margin: 0 15px 25px;
        display: none;
    }

    .room-joiner-wrapper {
        margin: 0 45px 25px;
    }
}

@include media-breakpoint-down(md) {
    .header-extra {
        .alert {
            display: initial;
        }

        .room-joiner {
            margin: 0 15px 25px;
        }
    }
}
</style>
