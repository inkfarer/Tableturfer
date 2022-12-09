<template>
    <div class="move-preview">
        <div class="background" />
        <div
            class="card-back"
            :class="`team-${props.team}`"
            :style="{ opacity: moveStore.nextMoveCompleted[props.team] ? 1 : 0 }"
        />
        <CardPreview
            :team="props.team"
            :name="move?.cardName"
            :style="{ opacity: moveVisible ? 1 : 0 }"
        />
    </div>
</template>

<script lang="ts" setup>
import { PropType } from 'vue';
import { PlayerTeam } from '~/types/PlayerTeam';
import { useMoveStore } from '~/stores/MoveStore';
import { computed, ref, watch } from '#imports';

const moveStore = useMoveStore();
const props = defineProps({
    team: {
        type: String as PropType<PlayerTeam>,
        required: true
    }
});

const move = computed(() => moveStore.lastMoves[props.team]);
const moveVisible = ref(false);
let moveClearTimeout: number | undefined;

watch(() => move.value, newMove => {
    if (newMove != null) {
        moveVisible.value = true;
        clearTimeout(moveClearTimeout);
        moveClearTimeout = window.setTimeout(() => {
            moveVisible.value = false;
        }, 5000);
    }
});
</script>

<style lang="scss" scoped>
.move-preview {
    position: relative;

    > .background {
        border-radius: 16px;
        background-color: rgba(0, 0, 0, 0.5);
        width: 100%;
        height: 100%;
        position: absolute;
        z-index: 1;
    }

    > .card-preview {
        position: relative;
        margin: 4px;
        z-index: 3;
        border-radius: 12px;
        transition: opacity 250ms;
    }

    > .card-back {
        position: absolute;
        z-index: 2;
        width: calc(100% - 8px);
        height: calc(100% - 8px);
        border-radius: 16px;
        margin: 4px;
        border-style: solid;
        border-width: 4px;
        box-sizing: border-box;
        transition: opacity 250ms;

        &.team-Bravo {
            border-color: rgba(21, 227, 219, 1);
            background: linear-gradient(to bottom, #4B50F3 0%, #1218EF 100%);
        }

        &.team-Alpha {
            border-color: #E9FF0F;
            background: linear-gradient(to bottom, #EC9009 0%, #C27707 100%);
        }
    }
}
</style>
