<script setup>

import { ref } from "vue";

import ViewSettingsGeneral from "./ViewSettingsGeneral.vue"


// ダイアログの状態（双方向バインディングを行う）
const showDialog = defineModel("showDialog");

// 項目一覧
const listItems = ref([
    { icon: "mdi-wrench", title: "General" },
    { icon: "mdi-information-outline", title: "About" },
]);

// 現在、選択されている項目（初期値はGeneral）
const selectedItem = ref("General");

// Walkのパラメータ（バックエンドに渡す）（双方向バインディングを行う）
const walkParams = defineModel("walkParams");

</script>

<template>
    <v-dialog v-model="showDialog" width="750">
        <v-card class="rounded-lg">
            <v-layout>
                <!-- 左のナビゲーションバー -->
                <v-navigation-drawer permanent floating class="bg-blue-grey-darken-3 text-white rounded-s-lg"
                    width="180">
                    <v-list>
                        <v-list-item v-for="item in listItems" :prepend-icon="item.icon" :title="item.title"
                            :active="item.title == selectedItem" @click="selectedItem = item.title"></v-list-item>
                    </v-list>
                </v-navigation-drawer>

                <v-main height="500" class="bg-blue-grey-darken-1 text-white enable-vertical-scroll">

                    <!-- 閉じるボタン -->
                    <v-app-bar color="transparent" height="40" elevation="0">
                        <template v-slot:append>
                            <v-icon color="white" icon="mdi-close" @click="showDialog = false" class="px-4"></v-icon>
                        </template>
                    </v-app-bar>

                    <!-- General -->
                    <v-container v-if="selectedItem == 'General'" fluid class="py-2">
                        <!-- 双方向バインディングを利用する -->
                        <ViewSettingsGeneral v-model:walkParams="walkParams"></ViewSettingsGeneral>
                    </v-container>

                    <!-- About -->
                    <v-container v-else-if="selectedItem == 'About'" fluid class="py-2">
                        <span>About</span>
                    </v-container>

                    <!-- 要素無し -->
                    <v-container v-else fluid class="py-2"></v-container>

                </v-main>
            </v-layout>
        </v-card>
    </v-dialog>
</template>

<style>
:root {
    /* blue-grey-darken-1 */
    --bg-color: #546E7A;
}

.enable-vertical-scroll {
    overflow-y: scroll;
}

/* スクロールバー全体の設定 */
.enable-vertical-scroll::-webkit-scrollbar {
    width: 25px;
}

/* スクロールバーのハンドル部分の設定 */
.enable-vertical-scroll::-webkit-scrollbar-track {
    /* 上下に余白を付ける */
    margin-top: 4px;
    margin-bottom: 4px;
}

/* スクロールバーの背景部分の設定 */
.enable-vertical-scroll::-webkit-scrollbar-thumb {
    background: var(--bg-color);
    border-radius: 15px;
    /* 左右に透明なボーダーをつける */
    border-right: 9px solid transparent;
    border-left: 9px solid transparent;
    /* 背景を切り取る */
    background-clip: padding-box;
}
</style>