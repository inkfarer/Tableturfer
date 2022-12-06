import ENTranslations from '../lang/game/en.json';
import { defineNuxtModule } from '@nuxt/kit';
import '@nuxtjs/i18n';

export default defineNuxtModule({
    async setup(options, nuxt) {
        nuxt.hook('i18n:extend-messages', async (additionalMessages) => {
            additionalMessages.push({
                en: {
                    game: {
                        ...ENTranslations,
                    }
                }
            });
        });
    }
});
