import { defineNuxtPlugin, useRuntimeConfig } from '#imports';
import { SocketService } from '~/plugins/SocketPlugin/SocketService';

export default defineNuxtPlugin(() => {
    const config = useRuntimeConfig();
    const socketService = new SocketService(config.public.socketUrl);

    return {
        provide: {
            socket: socketService
        }
    };
});
