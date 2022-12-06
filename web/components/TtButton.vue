<template>
    <a
        class="button"
        href="javascript:void(0);"
        @click="$emit('click')"
    >
        <slot />
    </a>
</template>

<style lang="scss" scoped>
.button {
    display: inline-block;
    color: white;
    text-decoration: none;
    font-size: 1.5em;
    font-weight: 600;
    background-color: rgba(0, 0, 0, 0.4);
    padding: 10px 15px;
    transition: color $default-transition-duration, background-color $default-transition-duration;

    &:hover {
        background-color: $accent;
        color: #222;
    }

    &:active {
        background-color: $accent-active;
        color: #222;
    }
}
</style>
