<template>
    <div class="alert">
        <slot />
    </div>
</template>

<style lang="scss" scoped>
.alert {
    padding: 8px;
    border-radius: 16px;
    border: 2px solid $error-red;
    background-color: $error-red-a20;
}
</style>
