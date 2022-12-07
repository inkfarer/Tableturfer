<template>
    <Overlay v-model="isOpen">
        <div class="deck-selector">
            <TtButton @click="setDeck(defaultDeck, 'default')">use default deck</TtButton>
            <TtButton @click="setDeck(testDeck, 'testDeck')">use other deck</TtButton>
        </div>
    </Overlay>
</template>

<script lang="ts" setup>
import { ref, useNuxtApp } from '#imports';
import { useDeckStore } from '~/stores/DeckStore';

const isOpen = ref(false);

function open() {
    isOpen.value = true;
}

const defaultDeck = [
    'ShooterNormal00',
    'BlasterMiddle00',
    'RollerNormal00',
    'ChargerNormal00',
    'SpinnerStandard00',
    'SlosherStrong00',
    'ManeuverNormal00',
    'StringerNormal00',
    'SaberLight00',
    'BombSplash',
    'Denchinamazu',
    'TakoDozer',
    'Shake',
    'Batoroika',
    'Mother'
];

const testDeck = [
    'ShooterBlaze00',
    'SlosherBathtub00',
    'Spiky',
    'Sutakoraa',
    'Taihou',
    'Yashiganisan',
    'Aori',
    'BombCurling',
    'ChargerNormal00',
    'HeroShooter',
    'Hotaru',
    'Ironic',
    'Iruka',
    'Jetpack',
    'Judgekun'
];

const deckStore = useDeckStore();
const { $socket } = useNuxtApp();

function setDeck(deck: string[], name: string) {
    deckStore.deckName = name;
    $socket.send('SetDeck', deck);
    isOpen.value = false;
}

defineExpose({
    open
});
</script>

<style lang="scss" scoped>
.deck-selector {
    .button {
        &:not(:last-child) {
            margin-bottom: 8px;
        }
    }
}
</style>
