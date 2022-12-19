import 'pinia';
import { Composer } from 'vue-i18n';

declare module 'pinia' {
    export interface PiniaCustomProperties {
        $i18n: Composer
    }
}
