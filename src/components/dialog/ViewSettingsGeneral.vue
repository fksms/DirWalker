<script setup>

import { ref, onMounted } from "vue";
import { open, message } from "@tauri-apps/plugin-dialog";

import i18n from "../i18n";


// Walkのパラメータ（バックエンドに渡す）（双方向バインディングを行う）
const walkParams = defineModel("walkParams");

// Walkのパラメータのクローン（ディープコピー）
const walkParamsClone = ref(JSON.parse(JSON.stringify(walkParams.value)));


// マウントされた後に行う処理
onMounted(() => {
    // オリジナル -> クローン
    walkParamsClone.value = JSON.parse(JSON.stringify(walkParams.value));
})


// Cancelボタンを押された時に行う処理
function canceled() {
    // オリジナル -> クローン
    walkParamsClone.value = JSON.parse(JSON.stringify(walkParams.value));
}


// Saveボタンを押された時に行う処理
async function saved() {

    // 空または空白の文字列を削除
    const newIgnoreDirectories = walkParamsClone.value.ignore_directories.filter(str => str.trim() !== "");

    // 1つだけ空文字を追加
    newIgnoreDirectories.push("");

    // 書き戻し（Deep copy）
    walkParamsClone.value.ignore_directories = newIgnoreDirectories.concat();

    // クローン -> オリジナル
    walkParams.value = JSON.parse(JSON.stringify(walkParamsClone.value));

    // Message Dialog
    await message(i18n.global.t("general.parameters_update"));
}


// Directory選択するDialogを開く関数
async function openDialog() {

    let defaultPath;

    // target_directoryが空文字の場合はnullを代入
    if (walkParamsClone.value.target_directory == "") {
        defaultPath = null;
    }
    else {
        defaultPath = walkParamsClone.value.target_directory;
    }

    // DialogをOpen
    const selectedPath = await open({
        title: "",
        defaultPath: defaultPath,
        multiple: false,
        directory: true,
        recursive: false
    });

    // nullではないかを判定
    if (selectedPath) {
        walkParamsClone.value.target_directory = selectedPath;
    }
}

// IgnoreDirectoryの追加
function addIgnore() {
    walkParamsClone.value.ignore_directories.push("");
}

</script>

<template>
    <h3>{{ $t("general.target_directory") }}</h3>
    <p class="text-grey-lighten-2">{{ $t("general.target_directory_desc") }}</p>
    <v-container fluid class="d-flex flex-row align-center px-0">
        <v-btn flat class="text-capitalize mr-4" color="blue-grey-lighten-1" text="Open" @click="openDialog()"></v-btn>
        <v-text-field v-model="walkParamsClone.target_directory" hide-details label="Path"></v-text-field>
    </v-container>

    <div class="py-2"></div>

    <h3>{{ $t("general.ignore_directories") }}</h3>
    <p class="text-grey-lighten-2">{{ $t("general.ignore_directories_desc") }}</p>
    <v-container fluid class="px-0">
        <v-row class="align-end">
            <v-col cols="1">
                <v-icon color="blue-grey-lighten-5" icon="mdi-plus-circle" class="pb-8" @click="addIgnore()"></v-icon>
            </v-col>
            <v-col>
                <!--
                v-for内で配列の要素をv-modelに使用する。
                https://zenn.dev/kokota/articles/d1a8582129c748
                 -->
                <div v-for="(item, index) in walkParamsClone.ignore_directories" :key="index">
                    <v-text-field v-model="walkParamsClone.ignore_directories[index]" hide-details single-line
                        density="compact" label="Path" class="pb-3"></v-text-field>
                </div>
            </v-col>
        </v-row>
    </v-container>

    <div class="py-1"></div>

    <v-container fluid class="d-flex flex-row align-center px-0">
        <v-spacer></v-spacer>
        <v-btn flat class="text-capitalize mr-4" color="blue-grey-lighten-1" text="Cancel" @click="canceled()"></v-btn>
        <v-btn flat class="text-capitalize" color="amber-darken-1" text="Save" @click="saved()"></v-btn>
    </v-container>
</template>

<style></style>