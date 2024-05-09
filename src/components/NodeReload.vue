<script setup>
import { invoke } from "@tauri-apps/api/tauri";


async function nodeReload() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  await invoke("node_reload", {})
  .then((success) => console.log(success))
  .catch((failure) => console.error(failure));
}
</script>

<template>
  <form class="row" @submit.prevent="nodeReload">
    <button type="submit">Node Reload</button>
  </form>
</template>
