<template>
    <div
        ref="root"
        class="room-code-input"
    >
        <input
            v-for="(item, index) in value"
            :key="`room-code-input-item_${index}`"
            :class="`room-code-input-item_${index}`"
            size="1"
            :value="item"
            placeholder="_"
            @input="updateValue($event, index)"
            @keydown="handleKeydown($event, index)"
        >
    </div>
</template>

<script lang="ts" setup>
import { ROOM_CODE_SIZE } from '~/data/Constants';
import { computed, ref } from 'vue';
import { isBlank } from '~/helpers/StringHelper';
import cloneDeep from 'lodash/cloneDeep';

const root = ref<HTMLDivElement | null>(null);
const emit = defineEmits(['update:modelValue']);
const props = defineProps({
    modelValue: {
        type: String,
        required: true
    }
});

const value = computed({
    get() {
        const result = new Array(ROOM_CODE_SIZE).fill(null);
        const modelValue = props.modelValue?.split('');
        if (modelValue != null) {
            result.splice(0, modelValue.length, ...modelValue);
        }

        return result;
    },
    set(value: Array<string | null>) {
        emit('update:modelValue', value.join(''));
    }
});

function blur() {
    root.value?.querySelectorAll('input').forEach(elem => elem.blur());
}

function selectLastInput(value: Array<string | null>) {
    const index = value.filter(item => item != null).length;
    if (index > ROOM_CODE_SIZE - 1) {
        blur();
    } else {
        (root.value?.querySelector(`.room-code-input-item_${index}`) as HTMLElement | null)?.focus();
    }
}

function updateValue(event: Event, index: number) {
    const target = event.target as HTMLInputElement;

    if (!isBlank(target.value)) {
        const result = cloneDeep(value.value);
        const normalizedValue = target.value.slice(0, value.value.length - index).split('');
        result.splice(index, normalizedValue.length, ...normalizedValue);
        value.value = result;

        selectLastInput(result);
    }

    event.preventDefault();
}

function handleKeydown(event: KeyboardEvent, index: number) {
    const target = event.target as HTMLInputElement;

    if (event.key === 'Backspace') {
        const result = cloneDeep(value.value);
        if (value.value[index] == null && index !== 0) {
            result[index - 1] = null;
        } else if (value.value[index] != null) {
            result[index] = null;
        }
        value.value = result;
        (target.previousElementSibling as HTMLElement | null)?.focus();
        event.preventDefault();
    } else if (value.value[index] != null && event.key !== 'Tab') {
        event.preventDefault();
    }
}
</script>

<style lang="scss" scoped>
.room-code-input {
    display: flex;

    > input {
        padding: 0;
        border-radius: 0;
        border: 0;
        width: 1.3em;
        border-bottom: 4px solid $accent;
        text-align: center;
        font-size: 3em;
        font-weight: 800;
        text-transform: uppercase;
        caret-color: transparent;
        background-color: $accent;
        color: #222;
        transition: background-color $default-transition-duration;

        &:not(:last-child) {
            margin-right: 8px;
        }

        &:focus {
            background-color: $accent-a50 !important;
        }

        &:placeholder-shown {
            color: white;
            background-color: rgba(0, 0, 0, 0.4);
        }

        &::placeholder {
            opacity: 0;
        }

        &:focus-visible {
            outline: 0;
        }
    }
}

@include media-breakpoint-down(sm) {
    .room-code-input > input {
        font-size: 10vw;
    }
}
</style>
