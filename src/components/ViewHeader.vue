<script setup>

import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";

// 親から渡されたコンポーネントの参照を受け取る
const props = defineProps(["viewSunburstChart"]);

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

// Walkデータ格納用
const walkData = ref();

// 受信メッセージ格納用（バックエンドから受け取る）
const progressMessage = ref("");

/* -------------------------------------------------------------------------- */

// ボタンの状態を保持
const buttonState = ref(false);
// ボタンの状態を変更
async function changeState() {
    buttonState.value = !buttonState.value;
    if (buttonState.value) {
        walkStart();
    } else {
        abort();
    }
};

// Walk Start
async function walkStart() {
    await invoke("walk_start", { str_params: JSON.stringify(walkParams.value) })
        // 成功した場合
        .then((success) => {
            // 受信メッセージを変更
            progressMessage.value = "Post-processing";

            // 受信したJSONをオブジェクトにデコード
            walkData.value = JSON.parse(success);

            // SVGエレメントを作成して格納
            props.viewSunburstChart.generateSunburst(walkData.value);

            // 受信メッセージを変更
            progressMessage.value = "Completion!";
        })
        // 失敗した場合
        .catch((failure) => {
            // エラーメッセージを出力
            progressMessage.value = "Error: " + failure;
            console.error(failure);
        });
    // ボタンの状態を戻す
    buttonState.value = false;
}

// Progress View
async function progressView() {
    await listen("ProgressNotification", event => {
        // 受信メッセージを格納
        progressMessage.value = event.payload;
    });
}
progressView();

// Abort
async function abort() {
    await invoke("abort", {})
    // ボタンの状態を戻す
    buttonState.value = true;
}

/* -------------------------------------------------------------------------- */


</script>


<template>
    <v-container fluid class="d-flex flex-row">

        <v-btn density="compact" :color="buttonState ? 'blue-grey-darken-1' : 'amber-darken-1'"
            :text="buttonState ? 'Abort' : 'Scan'" flat width="80" class="text-capitalize" @click="changeState">
        </v-btn>

        <span style="color:#ffffff" class="mx-5">
            {{ progressMessage }}
        </span>

    </v-container>
</template>