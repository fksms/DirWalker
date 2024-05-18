<script setup>

import { ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";

import { generateSunburst } from "./components/GenerateSunburst";

/* -------------------------------------------------------------------------- */

// Walkのパラメータ（バックエンドに渡す）
const walkParams = ref({
  target_directory: "",
  regex_filter: [],
  regex_invert_filter: [],
  ignore_directories: [],
  use_apparent_size: false,
});

walkParams.value.target_directory = "/Users/shogo/Downloads";
//walkParams.value.target_directory = "/Users/shogo";

// 受信メッセージ格納用（バックエンドから受け取る）
const progressMessage = ref("");

// ボタンの状態
const buttonState = ref(false);

// Walkデータ格納用
const walkData = ref(null);

// SVGSVGElement格納用
const svgContainer = ref(null);

// DOM要素への参照
const svgContainerRef = ref(null);

// svgContainerの変更を監視して、描画を更新する
watch(svgContainer, (newValue) => {
  const container = svgContainerRef.value;
  if (container && newValue) {
    // 既存の描画をクリア
    container.innerHTML = '';
    // 新しい SVGElement を追加
    container.appendChild(newValue);
  }
});

/* -------------------------------------------------------------------------- */

// Walk Start
async function walkStart() {
  await invoke("walk_start", { str_params: JSON.stringify(walkParams.value) })
    .then((success) => {
      // 成功した場合
      // 受信メッセージを変更
      progressMessage.value = "Post-processing";

      // 受信したJSONをオブジェクトにデコード
      walkData.value = JSON.parse(success);

      // SVGエレメントを作成して格納
      svgContainer.value = generateSunburst(walkData.value);

      // 受信メッセージを変更
      progressMessage.value = "Completion!";
    })
    .catch((failure) => {
      // 失敗した場合
      progressMessage.value = "Error!";
      console.error(failure);
    });
  buttonState.value = false;
}

// Abort
async function abort() {
  await invoke("abort", {})
  buttonState.value = true;
}

// Progress View
async function progressView() {
  await listen("ProgressNotification", event => {
    // 受信メッセージを格納
    progressMessage.value = event.payload;
  });
}
progressView();

// ステートを変更
async function changeState() {
  buttonState.value = !buttonState.value;
  if (buttonState.value) {
    walkStart();
  } else {
    abort();
  }
};

/* -------------------------------------------------------------------------- */

</script>

<template>
  <div style="background-color: #37474F; height: 100vh;">
    <v-container class="d-flex flex-row">

      <v-btn density="compact" :color="buttonState ? 'blue-grey-darken-1' : 'amber-darken-1'"
        :text="buttonState ? 'Abort' : 'Scan'" flat width="80" class="text-capitalize" @click="changeState">
      </v-btn>

      <span style="color:#ffffff" class="mx-5">
        {{ progressMessage }}
      </span>

    </v-container>

    <v-divider class="border-opacity-25" color="blue-grey-lighten-3"></v-divider>

    <v-container>
      <div ref="svgContainerRef" style="width: 70vh; height: 70vh;"></div>
    </v-container>

  </div>
</template>

<style></style>
