<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const walkParams = ref({
  target_directory: "",
  regex_filter: [],
  regex_invert_filter: [],
  ignore_directories: [],
  use_apparent_size: false,
});

walkParams.value.target_directory = "/Users/shogo/Downloads";
//walkParams.value.target_directory = "/Users/shogo";

async function walkStart() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  await invoke("walk_start", { str_params: JSON.stringify(walkParams.value) })
  .then((success) => console.log(success))
  .catch((failure) => console.error(failure));
}
</script>

<template>
  <form class="row" @submit.prevent="walkStart">
    <button type="submit">Walk Start</button>
  </form>
</template>
