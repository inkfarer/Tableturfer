<template>
    <div style="margin: 8px;">
        <div>hello</div>
        <NuxtLink to="/play">player</NuxtLink>
        <div>
            <button @click="createRoom">new room</button>
            <br>
            or
            <br>
            <input
                v-model="roomCode"
                placeholder="room code"
            >
            <button @click="joinRoom">join</button>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { navigateTo, ref } from '#imports';

const roomCode = ref('');

async function createRoom() {
    await navigateTo('/room/new');
}

async function joinRoom() {
    await navigateTo(`/room/${roomCode.value}`);
}
</script>
