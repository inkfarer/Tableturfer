<template>
    <div class="page-wrapper">
        <header>
            <PageNav class="width-cap" />
            <slot name="header" />
        </header>
        <main>
            <slot />
        </main>
        <footer>
            {{ $t('footer.text') }}
        </footer>
    </div>
</template>

<style lang="scss" scoped>
.page-wrapper {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    color: white;
}

header {
    background-size: 40px 40px;
    background-position-x: center;
    background-image:
        linear-gradient(to right, #262626 2px, transparent 2px),
        linear-gradient(to bottom, #262626 2px, transparent 2px);
}

main {
    flex-grow: 1;
    border-top: 2px solid $accent;
}

footer {
    border-top: 2px solid $accent;
    text-align: center;
    padding: 15px 0;
    background-color: #171717;
}
</style>
