<script setup>
import { ref } from 'vue';

import { detectOS } from '../lib/detectOS';

// 親から渡されたコンポーネントの参照を受け取る
const props = defineProps({
    sunburstChart: {
        type: Object,
        required: true,
    },
});

// 自身の情報
const ownColor = ref();
const ownName = ref();
const ownSize = ref();

// 子ノード
const children = ref([]);

// リストを作成（入力されたノードデータのchildrenをリストにして表示）
//
// node: ノードデータ
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

        // targetが"/"の時は"getLastPath"がfalseになるので、"node.data.name"を使う
        ownName.value = getLastPath(node.data.name) ? getLastPath(node.data.name) : node.data.name;
        ownColor.value = node.color;
        ownSize.value = array2String(toReadable(node.data.size));
    }

    // optionを指定する場合
    else {
        node.children.forEach((element) => {
            // 閾値よりも小さなもので配列を再構成する
            if (element.value <= option.threshold) {
                otherSize += element.value;
                children.value.push(element);
            }
        });

        // 名前無し
        ownName.value = null;
        ownColor.value = option.color;
        ownSize.value = array2String(toReadable(otherSize));
    }
}

// パスの最後の部分を取得
//
// path: ファイル・ディレクトリのパス（文字列）
function getLastPath(path) {
    // 分解した各パスを格納
    let segments = null;

    // Windowsの場合
    if (detectOS() == 'Windows') {
        // パスを"¥"で分割
        segments = path.split('\\');
    }
    // Windows以外の場合
    else {
        // パスを"/"で分割
        segments = path.split('/');
    }

    // 最後の要素を返す
    return segments[segments.length - 1];
}

// 配列から文字列に変換
function array2String(array) {
    // デリミタを指定
    const delimiter = ' ';
    return array.join(delimiter);
}

// TB/GB/MB/KBに変換
function toReadable(value) {
    return props.sunburstChart.toReadable(value);
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

// カーソルを合わせた時の動作
//
// node: ノードデータ
function mouseEntered(node) {
    return props.sunburstChart.mouseEntered(null, node);
}

// カーソルを離した時の動作
//
// node: ノードデータ
function mouseLeaved(node) {
    return props.sunburstChart.mouseLeaved(null, node);
}

// 外部から参照可能なプロパティを定義
defineExpose({
    generateDirectoryList,
    getLastPath,
});
</script>

<template>
    <v-table class="bg-transparent text-white" style="cursor: default">
        <!-- テーブルレイアウトでwidthの指定にピクセルとパーセントを混在させる場合は"colgroup"を利用する -->
        <!-- https://azuma006.hatenablog.com/entry/2020/01/31/233206 -->
        <colgroup>
            <col style="width: 40px" />
            <col style="width: auto" />
            <col style="width: 100px" />
        </colgroup>
        <tbody>
            <tr v-if="ownColor && ownSize">
                <th class="left-column"><v-icon :color="ownColor" icon="mdi-circle"></v-icon></th>
                <th class="center-column text-left">{{ ownName ? ownName : $t('directory_file_list.small_size_items') }}</th>
                <th class="right-column text-right">{{ ownSize }}</th>
            </tr>
        </tbody>
    </v-table>
    <!-- childrenの要素が10以下の場合はフッターを表示しない -->
    <v-data-table
        :items="children"
        density="compact"
        class="bg-transparent text-white"
        hover
        hide-no-data
        hide-default-header
        :hide-default-footer="children.length <= 10 ? true : false"
        :items-per-page="10"
    >
        <!-- テーブルレイアウトでwidthの指定にピクセルとパーセントを混在させる場合は"colgroup"を利用する -->
        <!-- https://azuma006.hatenablog.com/entry/2020/01/31/233206 -->
        <template #colgroup>
            <colgroup>
                <col style="width: 40px" />
                <col style="width: auto" />
                <col style="width: 100px" />
            </colgroup>
        </template>
        <template #item="{ item }">
            <tr @click.left="updateSunburst(item)" @click.right.prevent="showContextMenu(item)" @mouseenter="mouseEntered(item)" @mouseleave="mouseLeaved(item)">
                <td class="left-column"><v-icon :color="item.color" icon="mdi-circle-medium"></v-icon></td>
                <td class="center-column text-left">{{ getLastPath(item.data.name) }}</td>
                <td class="right-column text-right">{{ array2String(toReadable(item.data.size)) }}</td>
            </tr>
        </template>
    </v-data-table>
</template>

<style>
/* 左列 */
.left-column {
    /* 文字数が多い場合は省略しない */
    text-overflow: clip;
    /* 改行しない */
    white-space: nowrap;
    /* はみ出たまま表示 */
    overflow: visible;
    /* https://zenn.dev/milkandhoney995/articles/63eb778df55361 */
    max-width: 0;
    /* 上下のボーダーを削除 */
    border: none !important;
}

/* 中央列 */
.center-column {
    /* 文字数が多い場合は"..."で省略 */
    text-overflow: ellipsis;
    /* 改行しない */
    white-space: nowrap;
    /* はみ出た部分を隠して表示 */
    overflow: hidden;
    /* https://zenn.dev/milkandhoney995/articles/63eb778df55361 */
    max-width: 0;
    /* 上下のボーダーを削除 */
    border: none !important;
}

/* 右列 */
.right-column {
    /* 文字数が多い場合は"..."で省略 */
    text-overflow: ellipsis;
    /* 改行しない */
    white-space: nowrap;
    /* はみ出た部分を隠して表示 */
    overflow: hidden;
    /* https://zenn.dev/milkandhoney995/articles/63eb778df55361 */
    max-width: 0;
    /* 上下のボーダーを削除 */
    border: none !important;
}

/* リストホバー時のカラーとポインターを設定 */
tr:hover td {
    /* blue-grey-lighten-1 */
    background-color: #78909c;
    cursor: pointer;
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
