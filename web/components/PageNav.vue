<template>
    <nav>
        <div class="content-wrapper">
            <NuxtLink
                to="/"
                class="site-name"
            >
                {{ $t('nav.siteName') }}
            </NuxtLink>
            <div>
                <NuxtLink to="/decks">{{ $t('nav.decks') }}</NuxtLink>
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

    a {
        color: white;
        transition: color 350ms;
        text-decoration: none;
        font-size: 1.25em;

        &.site-name {
            font-size: 1.5em;
            font-weight: 800;
        }

        &:hover {
            color: #94DDC9;
        }
    }
}
</style>
