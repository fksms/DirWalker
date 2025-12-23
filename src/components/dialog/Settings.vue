<script setup>
import { ref, computed } from 'vue';

import i18n from '../../lib/i18n';
import { detectOS } from '../../lib/detectOS';
import SettingsGeneral from './SettingsGeneral.vue';
import SettingsLanguage from './SettingsLanguage.vue';
import SettingsPermissions from './SettingsPermissions.vue';
import SettingsAbout from './SettingsAbout.vue';

// ダイアログの状態（双方向バインディングを行う）
const showDialog = defineModel('showDialog', { type: Boolean });

// Walkのパラメータ（バックエンドに渡す）（双方向バインディングを行う）
const walkParams = defineModel('walkParams', { type: Object });

// 項目一覧
const listItems = computed(() => {
    return [
        { id: 1, icon: 'mdi-wrench', title: i18n.global.t('list_item.general'), visible: true },
        { id: 2, icon: 'mdi-earth', title: i18n.global.t('list_item.language'), visible: true },
        { id: 3, icon: 'mdi-lock-open-check', title: i18n.global.t('list_item.permissions'), visible: detectOS() == 'Mac' ? true : false },
        { id: 4, icon: 'mdi-information-outline', title: i18n.global.t('list_item.about'), visible: true },
    ].filter((item) => item.visible); // visibleがtrueのものでフィルタリング
});

// 現在、選択されている項目（初期値はid==1）
const selectedItem = ref(1);
</script>

<template>
    <v-dialog v-model="showDialog" width="750">
        <v-card class="rounded-lg">
            <v-layout>
                <!-- 左のナビゲーションバー -->
                <v-navigation-drawer permanent floating class="bg-blue-grey-darken-3 text-white rounded-s-lg" width="180">
                    <v-list>
                        <v-list-item
                            v-for="(item, index) in listItems"
                            :key="index"
                            :prepend-icon="item.icon"
                            :title="item.title"
                            :active="item.id == selectedItem"
                            @click="selectedItem = item.id"
                        ></v-list-item>
                    </v-list>
                </v-navigation-drawer>

                <!-- 右のメイン画面 -->
                <v-main height="500" class="bg-blue-grey-darken-1 text-white enable-vertical-scroll">
                    <!-- スクロールバー分のパディングを入れる -->
                    <div class="pl-5">
                        <!-- 閉じるボタン -->
                        <v-app-bar color="transparent" height="40" elevation="0">
                            <template #append>
                                <v-icon color="white" icon="mdi-close" class="px-4" @click="showDialog = false"></v-icon>
                            </template>
                        </v-app-bar>

                        <!-- General -->
                        <v-container v-if="selectedItem == 1" fluid class="py-0">
                            <!-- 双方向バインディングを利用する -->
                            <SettingsGeneral v-model:walk-params="walkParams"></SettingsGeneral>
                        </v-container>

                        <!-- Language -->
                        <v-container v-else-if="selectedItem == 2" fluid class="py-0">
                            <!-- 双方向バインディングを利用する -->
                            <SettingsLanguage></SettingsLanguage>
                        </v-container>

                        <!-- Permissions -->
                        <v-container v-else-if="selectedItem == 3" fluid class="py-0">
                            <!-- 双方向バインディングを利用する -->
                            <SettingsPermissions></SettingsPermissions>
                        </v-container>

                        <!-- About -->
                        <v-container v-else-if="selectedItem == 4" fluid class="py-0">
                            <!-- 双方向バインディングを利用する -->
                            <SettingsAbout></SettingsAbout>
                        </v-container>

                        <!-- 要素無し -->
                        <v-container v-else fluid class="py-0"></v-container>
                    </div>
                </v-main>
            </v-layout>
        </v-card>
    </v-dialog>
</template>

<style>
:root {
    /* blue-grey-darken-1 */
    --bg-color: #546e7a;
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
    margin-top: 40px;
    margin-bottom: 8px;
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
