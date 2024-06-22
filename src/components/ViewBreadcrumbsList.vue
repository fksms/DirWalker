<script setup>

import { ref } from "vue";

// 親から渡されたコンポーネントの参照を受け取る
const props = defineProps(["viewSunburstChart"]);


// 祖先ノード（自身も含む）
const ancestors = ref([]);


// パンくずリストを作成
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
function getLastPath(path) {
    // パスをスラッシュで分割
    const segments = path.split('/');
    // 最後の要素を返す
    return segments[segments.length - 1];
}


// Sunburstの更新
//
// node: ノードデータ
function updateSunburst(node) {
    // childrenが存在しない場合はupdateしない
    if (node.children) {
        return props.viewSunburstChart.leftClicked(node);
    }
}


// 外部から参照可能なプロパティを定義
defineExpose({
    generateBreadcrumbs,
});

</script>


<template>
    <v-container fluid class="pb-0">
        <ul class="breadcrumbs">
            <li v-for="(item, index) in ancestors">
                <a v-bind:class="{ 'current': index == ancestors.length - 1 }"
                    @click="(index != ancestors.length - 1) ? updateSunburst(item) : null">
                    {{ getLastPath(item.data.name) }}
                </a>
            </li>
        </ul>
    </v-container>
</template>


<style>
:root {
    /* blue-grey-darken-3 */
    --bg-color: #37474F;
    /* blue-grey-darken-1 */
    --breadcrumbs-bg-color: #546E7A;
    /* amber-darken-1 */
    --breadcrumbs-current-bg-color: #FFB300;
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
    width: 100%;
    -webkit-overflow-scrolling: touch;
}

.breadcrumbs::-webkit-scrollbar {
    height: 25px;
}

.breadcrumbs::-webkit-scrollbar-track {
    /* 左右に余白を付ける */
    margin-left: 4px;
    margin-right: 4px;
}

.breadcrumbs::-webkit-scrollbar-thumb {
    background: var(--breadcrumbs-bg-color);
    border-radius: 12.5px;
    /* 上下に透明なボーダーをつける */
    border-top: 9px solid transparent;
    border-bottom: 9px solid transparent;
    /* 背景を切り取る */
    background-clip: padding-box;
}

.breadcrumbs a {
    position: relative;
    padding: 1em 0.3em;
    background: var(--breadcrumbs-bg-color);
    color: white;
    cursor: pointer;
}

.breadcrumbs a::before {
    content: "";
    position: absolute;
    top: 50%;
    margin-top: -1.0em;
    border-width: 1.0em 0em 1.0em 1.0em;
    border-style: solid;
    border-color: var(--breadcrumbs-bg-color) transparent;
    left: -1em;
}

.breadcrumbs a::after {
    content: "";
    position: absolute;
    top: 50%;
    margin-top: -1.0em;
    border-top: 1.0em solid transparent;
    border-bottom: 1.0em solid transparent;
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