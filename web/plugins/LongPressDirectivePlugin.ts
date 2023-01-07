import { defineNuxtPlugin } from '#imports';

interface LongPressHandlerElement extends HTMLElement {
    dataset: {
        longPressHandlerId?: string
        longPressCompleted?: string
    }
}

export default defineNuxtPlugin(app => {
    app.vueApp.directive('longPress', {
        mounted(elem: LongPressHandlerElement, binding, vnode) {
            elem.style.touchAction = 'none';

            const callHandler = (name: string) => {
                if (vnode.props?.[name] != null) {
                    vnode.props?.[name]();
                }
            };

            const pointerUpListener = () => {
                if (elem.dataset.longPressCompleted === String(true)) {
                    elem.dataset.longPressCompleted = String(false);
                    callHandler('onLongPressStop');
                } else {
                    callHandler('onShortPress');
                }
                window.clearTimeout(parseInt(elem.dataset.longPressHandlerId ?? '0'));
            };

            const pointerDownListener = () => {
                const timeout = window.setTimeout(() => {
                    elem.dataset.longPressCompleted = String(true);
                    callHandler('onLongPressStart');
                }, 500);

                elem.dataset.longPressHandlerId = timeout.toString();
            };

            elem.addEventListener('pointerup', pointerUpListener);
            elem.addEventListener('pointerdown', pointerDownListener);
        },

        unmounted(elem: LongPressHandlerElement) {
            window.clearTimeout(elem.dataset.longPressHandlerId);
        }
    });
});
