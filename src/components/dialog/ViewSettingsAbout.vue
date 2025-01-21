<script setup>

import { ref, onMounted } from "vue";
import { getVersion } from "@tauri-apps/api/app";


// アプリのバージョン
const appVersion = ref("");


// マウントされた後に行う処理
onMounted(async () => {
    appVersion.value = await getVersion();
})

</script>

<template>
    <div style="text-align:center;">
        <img src="../assets/DirWalker.svg" style="height: 20vmin;"></img>
        <h2 class="pb-4">DirWalker</h2>
        <span class="pr-10">Version: {{ appVersion }}</span>
        <a href="https://github.com/fksms/DirWalker" target="_blank" class="text-white">GitHub</a>

        <p class="pt-12 pb-0">The MIT License</p>
        <p class="pt-0 pb-6">Copyright © 2024 Shogo Fukushima</p>
    </div>
    <div class="px-10">
        <p class="pb-6">{{ $t("about.lisense_1") }}</p>
        <p class="pb-6">{{ $t("about.lisense_2") }}</p>
        <p class="pb-6">{{ $t("about.lisense_3") }}</p>
    </div>
</template>

<style>
img {
    /* テキストの選択不可 */
    user-select: none;
    -webkit-user-select: none;
    -ms-user-select: none;
    -moz-user-select: none;
    -khtml-user-select: none;
    -webkit-user-drag: none;
    -khtml-user-drag: none;

    /* クリックしても反応しなくなる */
    pointer-events: none;
}
</style>