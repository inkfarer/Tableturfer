// https://v3.nuxtjs.org/api/configuration/nuxt.config
export default defineNuxtConfig({
    imports: {
        autoImport: false
    },
    modules: [
        '@pinia/nuxt'
    ],
    typescript: {
        typeCheck: true
    },
    runtimeConfig: {
        public: {
            socketUrl: 'ws://192.168.1.232:8080/ws'
        }
    }
});
