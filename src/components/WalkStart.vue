<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const dustParams = ref({
  target_directories: [],
  regex_filter: [],
  regex_invert_filter: [],
  ignore_directories: [],
  by_filecount: false,
  limit_filesystem: false,
  dereference_links: false,
  ignore_hidden_files: false,
  use_apparent_size: false,
});

dustParams.value.target_directories.push("/Users/shogo/Downloads");


async function walkStart() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  //console.log(dustParams);
  await invoke("walk_start", { str_params: JSON.stringify(dustParams.value) })
  .then((success) => console.log(success))
  .catch((failure) => console.error(failure));
}
</script>

<template>
  <form class="row" @submit.prevent="walkStart">
    <button type="submit">Start</button>
  </form>
</template>
