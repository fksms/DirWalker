<script setup>

import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

import i18n from "./i18n";
import { detectOS } from "./detectOS";
import ViewSettings from "./dialog/ViewSettings.vue";

// 親から渡されたコンポーネントの参照を受け取る
const props = defineProps(["viewSunburstChart"]);


// ボタンの状態を保持
const buttonState = ref(false);

// ダイアログの状態（双方向バインディングを行う）
const showDialog = ref(false);

// 受信メッセージ格納用（バックエンドから受け取る）
const statusMessage = ref("");

// Walkのパラメータ（バックエンドに渡す）（双方向バインディングを行う）
const walkParams = ref({
    target_directory: "",
    regex_filter: [],
    regex_invert_filter: [],
    ignore_directories: [],
    use_apparent_size: false,
});


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
        // ステータスの更新
        statusMessage.value = i18n.global.t("status_messages.os_error")
        // ボタンの状態を戻す
        buttonState.value = false;
        return;
    }

    // ターゲットディレクトリ未設定の場合
    if (walkParams.value.target_directory == "") {
        // ステータスの更新
        statusMessage.value = i18n.global.t("status_messages.not_set_target")
        // ボタンの状態を戻す
        buttonState.value = false;
        return;
    }

    // Walk Data
    let walkData = null;

    // エラーメッセージを格納
    let error = "";

    // Progressメッセージを受信するためのリスナーを起動
    const unlisten = await listen("ProgressNotification", event => {
        // デコード
        const progressNotification = JSON.parse(event.payload)

        // スキャン中
        if (progressNotification.scan_complete == false) {
            // スキャン済みのファイル総数
            const numFiles = progressNotification.num_files;
            // スキャン済みのファイル総サイズ
            const totalFileSize = progressNotification.total_file_size;
            // ステータスの更新
            statusMessage.value = `${i18n.global.t("status_messages.scanning")}  ${numFiles} files,  ${totalFileSize} bytes`;
        }

        // スキャン完了、後処理に移行
        else {
            // ステータスの更新
            statusMessage.value = i18n.global.t("status_messages.post_processing")
        }
    });

    // スキャン実行
    await invoke("walk_start", { str_params: JSON.stringify(walkParams.value) })
        // 成功した場合
        .then((success) => {
            walkData = success;
        })
        // 失敗した場合
        .catch((failure) => {
            error = failure;
        });

    // リスナーを停止
    unlisten();

    // エラーが発生した場合（"walkData"がnull）
    if (walkData == null) {
        // ステータスの更新
        statusMessage.value = `${i18n.global.t("status_messages.scan_error")} ${error}`
    }

    // 強制終了した場合（"walkData"が空）
    else if (walkData == "") {
        // ステータスの更新
        statusMessage.value = i18n.global.t("status_messages.aborted")
    }

    // 正常に受信できた場合
    else {
        // Sunburstの作成
        await generateSunburst(JSON.parse(walkData));
        // ステータスの更新
        statusMessage.value = i18n.global.t("status_messages.completed")
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


// ボタンの状態を変更
function changeState() {
    buttonState.value = !buttonState.value;
    if (buttonState.value) {
        walkStart();
    } else {
        abort();
    }
};


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
            {{ statusMessage }}
        </span>

        <v-spacer></v-spacer>

        <v-icon color="blue-grey-lighten-5" icon="mdi-cog" @click="showDialog = true"></v-icon>
    </v-container>

    <!-- 双方向バインディングを利用する -->
    <ViewSettings v-model:showDialog="showDialog" v-model:walkParams="walkParams"></ViewSettings>
</template>

<style></style>