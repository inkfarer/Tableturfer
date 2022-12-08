<template>
    <TtButton
        :disabled="activeCardStore.locked"
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
</script>
