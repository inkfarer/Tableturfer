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
            <i18n-t
                keypath="footer.createdBy"
                tag="div"
            >
                <NuxtLink to="https://twitter.com/inkfarer">inkfarer</NuxtLink>
            </i18n-t>
            <div
                class="version-info"
                :class="{ 'is-prod': isProd }"
            >
                <i18n-t keypath="footer.appInfo.template">
                    <template #name>
                        <NuxtLink :to="config.public.repositoryUrl">{{ $t('footer.appInfo.name') }}</NuxtLink>
                    </template>
                    <template #details>
                        <i18n-t
                            v-if="isProd"
                            keypath="footer.appInfo.buildInfo"
                        >
                            <template #commit>
                                <NuxtLink :to="`${config.public.repositoryUrl}/commit/${config.public.commitHash}`">{{ config.public.commitHash }}</NuxtLink>
                            </template>
                            <template #date>
                                {{ formatDate(config.public.buildDate) }}
                            </template>
                        </i18n-t>
                        <span
                            v-else
                            class="dev-mode"
                        >
                            {{ $t('footer.appInfo.devBuild') }}
                        </span>
                    </template>
                </i18n-t>
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
    background-color: $page-background;

    > .version-info {
        font-size: 0.75em;
        margin-top: 4px;

        a {
            color: white;
            text-decoration: none;

            &:hover {
                text-decoration: underline;
            }
        }

        &.is-prod {
            opacity: 0.5;
        }

        .dev-mode {
            color: $error-red;
        }
    }
}
</style>
