<template>
    <details class="tt-details">
        <slot />
    </details>
</template>

<style lang="scss" scoped>
details {
    border-radius: 16px;
    border: 2px solid $info-blue;
    background-color: $info-blue-a20;
    padding: 8px 12px;
    text-align: left;
}
</style>

<style lang="scss">
details.tt-details[open] > summary {
    margin-bottom: 5px;
}

details.tt-details > summary {
    cursor: pointer;
}
</style>
