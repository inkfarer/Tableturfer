<template>
    <div>
        <button @click="setDeck(defaultDeck)">use default deck</button>
        <button @click="setDeck(testDeck)">use other deck</button>
    </div>
</template>

<script lang="ts" setup>
import { useNuxtApp } from '#imports';

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

const { $socket } = useNuxtApp();

function setDeck(deck: string[]) {
    $socket.send('SetDeck', deck);
}
</script>
