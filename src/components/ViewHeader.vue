<script setup>

import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";

import { detectOS } from "./DetectOS";
import ViewSettings from "./dialog/ViewSettings.vue";

// 親から渡されたコンポーネントの参照を受け取る
const props = defineProps(["viewSunburstChart"]);


// ボタンの状態を保持
const buttonState = ref(false);

// ダイアログの状態（双方向バインディングを行う）
const showDialog = ref(false);

// 受信メッセージ格納用（バックエンドから受け取る）
const progressMessage = ref("");

// Walkのパラメータ（バックエンドに渡す）（双方向バインディングを行う）
const walkParams = ref({
    target_directory: "",
    regex_filter: [],
    regex_invert_filter: [],
    ignore_directories: [],
    use_apparent_size: false,
});


// ボタンの状態を変更
function changeState() {
    buttonState.value = !buttonState.value;
    if (buttonState.value) {
        walkStart();
    } else {
        abort();
    }
};


// マウントされた後に行う処理
onMounted(() => {
    // Windowsの場合
    if (detectOS() == "Windows") {
        walkParams.value.target_directory = "C:\\";
        walkParams.value.ignore_directories.push("");
    }
    // Macの場合
    else if (detectOS() == "Mac") {
        walkParams.value.target_directory = "/";
        walkParams.value.ignore_directories.push("/System/Volumes");
        walkParams.value.ignore_directories.push("");
    }
    // Linuxの場合
    else if (detectOS() == "Linux") {
        walkParams.value.target_directory = "/";
        walkParams.value.ignore_directories.push("");
    }
    // それ以外
    else { }
})


// Walk Start
async function walkStart() {

    // OS対象外の場合は終了
    if (detectOS() == null) {
        progressMessage.value = "OS Error";
        return;
    }

    // ターゲットディレクトリ未設定の場合
    if (walkParams.value.target_directory == "") {
        progressMessage.value = "Please set the target directory.";
        return;
    }

    // Walk Data
    let walkData = null;

    // エラーメッセージを格納
    let error = "";

    // Progressメッセージを受信するためのリスナーを起動
    const unlisten = await listen("ProgressNotification", event => {
        // 受信メッセージを格納
        progressMessage.value = event.payload;
    });

    // バックエンド側の関数を実行
    await invoke("walk_start", { str_params: JSON.stringify(walkParams.value) })
        // 成功した場合
        .then((success) => {
            walkData = success;
        })
        // 失敗した場合
        .catch((failure) => {
            error = "Error: " + failure;
        });

    // リスナーを停止
    unlisten();

    // エラーが発生した場合
    if (walkData == null) {
        progressMessage.value = error;
    }

    // 強制終了した場合
    else if (walkData == "") {
        progressMessage.value = "Abort!";
    }

    // 正常に受信できた場合
    else {
        // Sunburstの作成
        await generateSunburst(JSON.parse(walkData));
        progressMessage.value = "Completion!";
    }

    // ボタンの状態を戻す
    buttonState.value = false;
}


// Abort
async function abort() {
    await invoke("abort", {})
    // ボタンの状態を戻す
    buttonState.value = true;
}


// Sunburstの作成
async function generateSunburst(data) {
    return props.viewSunburstChart.generateSunburst(data);
}

</script>

<template>
    <v-container fluid class="d-flex flex-row">

        <v-btn density="compact" :color="buttonState ? 'blue-grey-darken-1' : 'amber-darken-1'"
            :text="buttonState ? 'Abort' : 'Scan'" flat width="80" class="text-capitalize" @click="changeState()">
        </v-btn>

        <span class="mx-5 text-white" style="cursor: default;">
            {{ progressMessage }}
        </span>

        <v-spacer></v-spacer>

        <v-icon color="blue-grey-lighten-5" icon="mdi-cog" @click="showDialog = true"></v-icon>
    </v-container>

    <!-- 双方向バインディングを利用する -->
    <ViewSettings v-model:showDialog="showDialog" v-model:walkParams="walkParams"></ViewSettings>
</template>

<style></style>