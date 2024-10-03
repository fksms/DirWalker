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
        <p class="pb-6">
            Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated
            documentation files (the “Software”), to deal in the Software without restriction, including without
            limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
            the Software, and to permit persons to whom the Software is furnished to do so, subject to the following
            conditions:
        </p>
        <p class="pb-6">
            The above copyright notice and this permission notice shall be included in all copies or substantial
            portions of the Software.
        </p>
        <p class="pb-6">
            THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT
            LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO
            EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN
            AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE
            OR OTHER DEALINGS IN THE SOFTWARE.
        </p>
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