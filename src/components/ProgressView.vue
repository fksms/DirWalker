<script setup>
import { ref } from "vue";
import { listen } from "@tauri-apps/api/event";

const progressMessage = ref("");

async function progressView() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  await listen("ProgressNotification", event => {
    progressMessage.value = event.payload;
  });
}
progressView();
</script>

<template>
  <p>{{ progressMessage }}</p>
</template>