<template>
    <div>
        <div>
            active card: {{ activeCardStore.activeCard?.name ?? 'None' }}
        </div>
        <div>
            pos: {{ activeCardStore.internalPosition }}
        </div>
        <div>
            pos2: {{ activeCardStore.position }}
        </div>
        <div>
            rot: {{ activeCardStore.rotation }}
        </div>
    </div>
</template>

<script lang="ts" setup>
import { useActiveCardStore } from '~/stores/ActiveCardStore';

const activeCardStore = useActiveCardStore();
</script>
