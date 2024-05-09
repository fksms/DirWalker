<script setup>
import { invoke } from "@tauri-apps/api/tauri";


async function abort() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  await invoke("abort", {})
}
</script>

<template>
  <form class="row" @submit.prevent="abort">
    <button type="submit">Abort</button>
  </form>
</template>
