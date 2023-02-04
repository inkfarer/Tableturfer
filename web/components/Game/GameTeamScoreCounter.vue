<template>
    <div
        class="score-counter"
        :class="`team-${props.team}`"
    >
        {{ score }}
    </div>
</template>

<script lang="ts" setup>
import { PlayerTeam } from '~/types/PlayerTeam';
import { useRoomStore } from '~/stores/RoomStore';
import { computed } from '#imports';

const props = defineProps<{
    team: PlayerTeam
}>();

const roomStore = useRoomStore();
const score = computed(() => roomStore.score?.[props.team] ?? 0);
</script>

<style lang="scss" scoped>
.score-counter {
    text-align: center;
    font-size: 2.5em;
    font-weight: 800;
    border-style: solid;
    border-width: 2px;
    padding: 0.25em 0;
    min-width: 1.2em;
    font-feature-settings: 'tnum';

    &.team-Alpha {
        background-color: $accent-alpha-a10;
        border-color: $accent-alpha;
    }

    &.team-Bravo {
        background-color: $accent-bravo-a10;
        border-color: $accent-bravo;
    }
}

@include media-breakpoint-down(lg) {
    .score-counter {
        padding: 0.25em 0.25em;
    }
}

@include media-breakpoint-down(md) {
    .score-counter {
        font-size: 1.75em;
        padding: 0.1em 0.25em;
    }
}
</style>
