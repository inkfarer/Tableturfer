import { gitDescribeSync } from 'git-describe';

const gitInfo = gitDescribeSync();

// https://v3.nuxtjs.org/api/configuration/nuxt.config
export default defineNuxtConfig({
    vite: {
        css: {
            preprocessorOptions: {
                scss: {
                    additionalData: '@use "@/assets/styles/constants.scss" as *; @use "@/assets/styles/breakpoints.scss" as *;'
                }
            }
        }
    },
    css: [
        '@/assets/styles/style-common.scss',
        '/node_modules/normalize.css/normalize.css'
    ],
    imports: {
        autoImport: false
    },
    modules: [
        '@pinia/nuxt',
        '~/modules/CardI18nModule.ts',
        '@nuxtjs/i18n',
        'nuxt-icon'
    ],
    typescript: {
        typeCheck: true
    },
    i18n: {
        locales: [
            {
                code: 'en',
                file: 'en.json'
            }
        ],
        lazy: true,
        langDir: 'lang',
        defaultLocale: 'en',
        strategy: 'no_prefix',
        vueI18n: {
            legacy: false,
            locale: 'en'
        }
    },
    runtimeConfig: {
        public: {
            socketUrl: 'ws://192.168.1.232:8080/ws',
            commitHash: gitInfo.hash,
            buildDate: new Date().getTime(),
            repositoryUrl: 'https://github.com/inkfarer/Tableturfer'
        }
    }
});
