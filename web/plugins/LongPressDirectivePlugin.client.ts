import { defineNuxtPlugin } from '#imports';
import { parseInt } from 'lodash';

interface LongPressHandlerElement extends HTMLElement {
    dataset: {
        longPressHandlerId?: string
        longPressCompleted?: string
    }
}

const clickEvent = new CustomEvent('click', { detail: { isLongPressEvent: true } });
const longPressStopEvent = new CustomEvent('long-press-stop');
const longPressStartEvent = new CustomEvent('long-press-start');

export default defineNuxtPlugin(app => {
    app.vueApp.directive('longPress', {
        created(elem: LongPressHandlerElement, binding, vnode) {
            elem.style.touchAction = 'none';

            const pointerUpListener = () => {
                if (elem.dataset.longPressCompleted === String(true)) {
                    elem.dataset.longPressCompleted = String(false);
                    if (vnode.component) {
                        vnode.component.emit('long-press-stop');
                    } else {
                        elem.dispatchEvent(longPressStopEvent);
                    }
                } else {
                    if (vnode.component) {
                        vnode.component.emit('click');
                    } else {
                        elem.dispatchEvent(clickEvent);
                    }
                }
                window.clearTimeout(parseInt(elem.dataset.longPressHandlerId ?? '0'));
                document.removeEventListener('pointerup', pointerUpListener);
            };

            const pointerDownListener = () => {
                document.addEventListener('pointerup', pointerUpListener);

                const timeout = window.setTimeout(() => {
                    elem.dataset.longPressCompleted = String(true);
                    if (vnode.component) {
                        vnode.component.emit('long-press-start');
                    } else {
                        elem.dispatchEvent(longPressStartEvent);
                    }
                }, 500);

                elem.dataset.longPressHandlerId = timeout.toString();
            };

            const clickListener = (event: MouseEvent | typeof clickEvent) => {
                if (typeof event.detail !== 'object' || !event.detail.isLongPressEvent) {
                    event.stopImmediatePropagation();
                    event.preventDefault();
                }
            };

            elem.addEventListener('pointerdown', pointerDownListener);
            elem.addEventListener('click', clickListener);
        },

        unmounted(elem: LongPressHandlerElement) {
            window.clearTimeout(elem.dataset.longPressHandlerId);
        }
    });
});
