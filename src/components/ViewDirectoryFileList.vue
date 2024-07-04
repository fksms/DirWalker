<script setup>

import { ref } from "vue";

import "@mdi/font/css/materialdesignicons.css";

// 親から渡されたコンポーネントの参照を受け取る
const props = defineProps(["viewSunburstChart"]);


// 自身の情報
const ownColor = ref();
const ownName = ref();
const ownSize = ref();

// 子ノード
const children = ref([]);


// リストを作成
//
// node: カーソルを合わせた円弧or円のデータ
// option: オプション
function generateDirectoryList(node, option) {

    // ファイルサイズカウント用
    let otherSize = 0;

    // 配列を初期化
    children.value.splice(0);

    // optionを指定しない場合
    if (option == null) {
        // childrenがnullではない場合
        if (node.children != null) {
            // Deep copy
            children.value = node.children.concat();
        }

        ownColor.value = node.color;
        ownName.value = getLastPath(node.data.name);
        ownSize.value = array2String(toReadable(node.data.size));
    }

    // optionを指定する場合
    else {
        node.children.forEach(element => {
            // 閾値よりも小さなもので配列を再構成する
            if (element.value <= option.threshold) {
                otherSize += element.value;
                children.value.push(element);
            }
        });

        ownColor.value = option.color;
        ownName.value = "Other small size items";
        ownSize.value = array2String(toReadable(otherSize));
    }
}


// パスの最後の部分を取得
function getLastPath(path) {
    // パスをスラッシュで分割
    const segments = path.split('/');
    // 最後の要素を返す
    return segments[segments.length - 1];
}


// 配列から文字列に変換
function array2String(array) {
    // デリミタを指定
    const delimiter = " "
    return array.join(delimiter);
}


// TB/GB/MB/KBに変換
function toReadable(value) {
    return props.viewSunburstChart.toReadable(value);
}


// Sunburstの更新
//
// node: ノードデータ
function updateSunburst(node) {
    return props.viewSunburstChart.leftClicked(node);
}


// コンテキストメニューの表示
//
// node: ノードデータ
function showContextMenu(node) {
    return props.viewSunburstChart.rightClicked(node);
}


// カーソルを合わせた時の動作
//
// node: ノードデータ
function mouseEntered(node) {
    return props.viewSunburstChart.mouseEntered(null, node, null);
}


// カーソルを離した時の動作
//
// node: ノードデータ
function mouseLeaved(node) {
    return props.viewSunburstChart.mouseLeaved(null, node);
}


// 外部から参照可能なプロパティを定義
defineExpose({
    generateDirectoryList,
});

</script>

<template>
    <v-table class="bg-transparent text-white cursor-default">
        <tbody>
            <tr v-if="ownColor && ownName && ownSize">
                <th width="36"><v-icon :color="ownColor">mdi-circle</v-icon></th>
                <th width="auto" nowrap class="text-left">{{ ownName }}</th>
                <th width="100" nowrap class="text-right">{{ ownSize }}</th>
            </tr>
        </tbody>
    </v-table>
    <v-data-table :items="children" density="compact" class="bg-transparent text-white" hover hide-no-data
        hide-default-header :hide-default-footer="(children.length <= 10) ? true : false" :items-per-page="10">
        <template v-slot:item="{ item }">
            <tr @click.left="updateSunburst(item)" @click.right.prevent="showContextMenu(item)"
                @mouseenter="mouseEntered(item)" @mouseleave="mouseLeaved(item)">
                <td width="36"><v-icon :color="item.color">mdi-circle-medium</v-icon></td>
                <td width="auto" nowrap class="text-left">{{ getLastPath(item.data.name) }}</td>
                <td width="100" nowrap class="text-right">{{ array2String(toReadable(item.data.size)) }}</td>
            </tr>
        </template>
    </v-data-table>
</template>


<style>
td,
th {
    /* 文字数が多い場合は省略 */
    text-overflow: ellipsis;
    white-space: nowrap;
    overflow: hidden;
    max-width: 0;
    /* 上下のボーダーを削除 */
    border: none !important;
}

tr:hover td {
    /* blue-grey-lighten-1 */
    background-color: #78909C;
    cursor: pointer;
}

.cursor-default {
    cursor: default;
}

/* フッターの上にある水平線を透明に設定 */
hr.v-divider {
    color: transparent;
}

/* VDataTableのitems-per-page dropdownを非表示 */
.v-data-table-footer__items-per-page {
    display: none !important;
}

/* フッター上のカーソルをdefaultに設定 */
.v-data-table-footer__info {
    cursor: default;
}
</style>