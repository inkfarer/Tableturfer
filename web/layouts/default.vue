<template>
    <div class="page-wrapper">
        <header>
            <PageNav class="width-cap" />
            <slot name="header" />
        </header>
        <main>
            <slot />
        </main>
        <footer>
            <div>{{ $t('footer.beforeCreatedBy') }} <NuxtLink to="https://twitter.com/inkfarer">inkfarer</NuxtLink>.</div>
            <div
                class="version-info"
                :class="{ 'is-prod': isProd }"
            >
                <template v-if="isProd">
                    {{ `${$t('footer.versionInfo.appName')} ${$t('footer.versionInfo.commitAndDate', { commit: config.public.commitHash, date: formatDate(config.public.buildDate) })}` }}
                </template>
                <template v-else>
                    {{ $t('footer.versionInfo.appName') }} <span class="dev-mode">{{ $t('footer.versionInfo.devBuild') }}</span>
                </template>
            </div>
        </footer>
    </div>
</template>

<script lang="ts" setup>
import { useRuntimeConfig } from '#imports';
import { formatDate } from '#imports';

const isProd = process.env.NODE_ENV === 'production';
const config = useRuntimeConfig();
</script>

<style lang="scss" scoped>
.page-wrapper {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
}

header {
    background-size: 40px 40px;
    background-position-x: center;
    background-image:
        linear-gradient(to right, #262626 2px, transparent 2px),
        linear-gradient(to bottom, #262626 2px, transparent 2px);
}

main {
    flex-grow: 1;
    border-top: 2px solid $accent;
    display: flex;
    flex-direction: column;
}

footer {
    border-top: 2px solid $accent;
    text-align: center;
    padding: 15px 0;
    background-color: #171717;

    > .version-info {
        font-size: 0.75em;
        margin-top: 4px;

        &.is-prod {
            opacity: 0.5;
        }

        .dev-mode {
            color: $error-red;
        }
    }
}
</style>
