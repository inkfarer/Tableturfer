<template>
    <div class="data-row-wrapper">
        <div class="data-row">
            <slot />
            <span class="label">{{ props.label }}</span>
        </div>
    </div>
</template>

<script lang="ts" setup>
const props = defineProps({
    label: {
        type: String,
        required: true
    }
});
</script>

<style lang="scss" scoped>
.data-row-wrapper {
    overflow: hidden;
    min-height: 25px;
    display: flex;
    flex-direction: column;
    align-items: flex-end;

    &:not(:first-child) {
        margin-top: 10px;
    }
}

.data-row {
    border-bottom: 2px solid $accent;
    text-align: right;
    overflow-wrap: anywhere;
    width: 100%;

    > .label {
        overflow-wrap: initial;
        float: left;
        font-size: 0.75em;
        font-weight: 400;
        color: $accent;
        display: flex;
        align-items: flex-end;
        height: 100%;
        margin-right: 5px;
        line-height: 20px;
    }
}
</style>
