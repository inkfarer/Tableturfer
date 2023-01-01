<template>
    <div class="overlay">
        <slot />
    </div>
</template>

<style lang="scss" scoped>
.overlay {
    margin: 0 auto;
    padding: 10px;
    border-radius: 16px;
    background-color: #262626;
    border: 2px solid $accent;
    min-width: 200px;
    display: block;
    max-height: 80%;
    overflow-y: auto;
}
</style>
