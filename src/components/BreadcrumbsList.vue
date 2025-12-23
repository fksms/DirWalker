<script setup>
import { ref } from 'vue';

// 親から渡されたコンポーネントの参照を受け取る
const props = defineProps({
    sunburstChart: {
        type: Object,
        required: true,
    },
    directoryFileList: {
        type: Object,
        required: true,
    },
});

// 祖先ノード（自身も含む）
const ancestors = ref([]);

// パンくずリストを作成（入力されたノードデータのancestorsをパンくずリストにして表示）
//
// node: ノードデータ
function generateBreadcrumbs(node) {
    // 配列を初期化
    ancestors.value.splice(0);

    // nodeがnullになるまで繰り返し
    while (node != null) {
        // 配列の先頭に追加
        ancestors.value.unshift(node);
        // 親に移動
        node = node.parent;
    }
}

// パスの最後の部分を取得
//
// path: ファイル・ディレクトリのパス（文字列）
function getLastPath(path) {
    return props.directoryFileList.getLastPath(path);
}

// Sunburstの更新
//
// node: ノードデータ
function updateSunburst(node) {
    return props.sunburstChart.leftClicked(node);
}

// コンテキストメニューの表示
//
// node: ノードデータ
function showContextMenu(node) {
    return props.sunburstChart.rightClicked(node);
}

// 外部から参照可能なプロパティを定義
defineExpose({
    generateBreadcrumbs,
});
</script>

<template>
    <v-container fluid class="pb-0">
        <ul class="breadcrumbs enable-horizontal-scroll">
            <li v-for="(item, index) in ancestors" :key="index">
                <!-- 
                ancestorsの一番後ろの要素に"current"クラスを設定する
                ancestorsの一番後ろの要素を左クリックした場合は何もしない
                -->
                <a :class="{ current: index == ancestors.length - 1 }" @click.left="index != ancestors.length - 1 ? updateSunburst(item) : null" @click.right.prevent="showContextMenu(item)">
                    <!-- ルートディレクトリはフルパスで表示する -->
                    {{ index == 0 ? item.data.name : getLastPath(item.data.name) }}
                </a>
            </li>
        </ul>
    </v-container>
</template>

<style>
:root {
    /* blue-grey-darken-3 */
    --bg-color: #37474f;
    /* blue-grey-darken-1 */
    --breadcrumbs-bg-color: #546e7a;
    /* amber-darken-1 */
    --breadcrumbs-current-bg-color: #ffb300;
}

/* スクロールバー全体の設定 */
.enable-horizontal-scroll::-webkit-scrollbar {
    height: 25px;
}

/* スクロールバーのハンドル部分の設定 */
.enable-horizontal-scroll::-webkit-scrollbar-track {
    /* 左右に余白を付ける */
    margin-left: 4px;
    margin-right: 4px;
}

/* スクロールバーの背景部分の設定 */
.enable-horizontal-scroll::-webkit-scrollbar-thumb {
    background: var(--breadcrumbs-bg-color);
    border-radius: 15px;
    /* 上下に透明なボーダーをつける */
    border-top: 9px solid transparent;
    border-bottom: 9px solid transparent;
    /* 背景を切り取る */
    background-clip: padding-box;
}

ul {
    margin: 0;
    padding: 0;
    list-style: none;
}

li {
    margin: 0em 0.8em 0em 0.8em;
    display: inline-block;
}

.breadcrumbs {
    white-space: nowrap;
    overflow: hidden;
    overflow-x: scroll;
}

.breadcrumbs a {
    position: relative;
    padding: 1em 0.3em;
    background: var(--breadcrumbs-bg-color);
    color: white;
    cursor: pointer;
}

.breadcrumbs a::before {
    content: '';
    position: absolute;
    top: 50%;
    margin-top: -1em;
    border-width: 1em 0em 1em 1em;
    border-style: solid;
    border-color: var(--breadcrumbs-bg-color) transparent;
    left: -1em;
}

.breadcrumbs a::after {
    content: '';
    position: absolute;
    top: 50%;
    margin-top: -1em;
    border-top: 1em solid transparent;
    border-bottom: 1em solid transparent;
    border-left: 1em solid var(--breadcrumbs-bg-color);
    right: -1em;
}

.breadcrumbs a.current {
    color: var(--bg-color);
    font-weight: 500;
    background: var(--breadcrumbs-current-bg-color);
}

.breadcrumbs a.current::before {
    border-color: var(--breadcrumbs-current-bg-color) transparent;
}

.breadcrumbs a.current::after {
    border-left-color: var(--breadcrumbs-current-bg-color);
}
</style>
