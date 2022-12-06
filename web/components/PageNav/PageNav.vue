<template>
    <nav>
        <div class="content-wrapper">
            <PageNavLink
                to="/"
                class="site-name"
            >
                {{ $t('nav.siteName') }}
            </PageNavLink>
            <div>
                <PageNavLink to="/decks">{{ $t('nav.decks') }}</PageNavLink>
            </div>
        </div>
    </nav>
</template>

<style lang="scss" scoped>
.content-wrapper {
    margin: 1em 0;
    background-color: rgba(10, 10, 10, 0.9);
    border-radius: 8px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 10px 20px;

    a.site-name {
        font-size: 1.5em;
        font-weight: 800;
    }
}
</style>
