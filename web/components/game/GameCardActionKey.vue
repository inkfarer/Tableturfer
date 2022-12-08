<template>
    <TtButton
        :disabled="disabled"
        @click="emit('click')"
    >
        <Icon
            v-if="props.icon != null"
            :name="props.icon"
        />
        <template v-else>
            {{ props.text }}
        </template>
    </TtButton>
</template>

<script lang="ts" setup>
import { useActiveCardStore } from '~/stores/ActiveCardStore';
import { PropType } from 'vue';
import { computed } from '#imports';

const emit = defineEmits(['click']);

const props = defineProps({
    icon: {
        type: String as PropType<string | null>,
        default: null
    },
    text: {
        type: String as PropType<string | null>,
        default: null
    }
});

const activeCardStore = useActiveCardStore();
const disabled = computed(() => activeCardStore.locked || activeCardStore.pass);
</script>
