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
const children = ref();


// リストを作成
function generateDirectoryList(node) {
  ownColor.value = node.color;
  ownName.value = node.data.name;
  ownSize.value = node.data.size;
  children.value = node.children;
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
  // childrenが存在しない場合はupdateしない
  if (node.children) {
    return props.viewSunburstChart.leftClicked(node);
  }
}


// カーソルを合わせた時の動作
//
// node: ノードデータ
function mouseEntered(node) {
  return props.viewSunburstChart.mouseEntered(null, node);
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
        <th width="10%"><v-icon :color="ownColor">mdi-circle</v-icon></th>
        <th width="65%" nowrap class="text-left">{{ getLastPath(ownName) }}</th>
        <th width="25%" nowrap class="text-right">{{ array2String(toReadable(ownSize)) }}</th>
      </tr>
    </tbody>
  </v-table>
  <v-table density="compact" class="bg-transparent text-white" hover height="360">
    <tbody>
      <tr v-for="item in children" v-bind:key="item" @click="updateSunburst(item)" @mouseenter="mouseEntered(item)"
        @mouseleave="mouseLeaved(item)">
        <td width="10%"><v-icon :color="item.color">mdi-circle-medium</v-icon></td>
        <td width="65%" nowrap class="text-left">{{ getLastPath(item.data.name) }}</td>
        <td width="25%" nowrap class="text-right">{{ array2String(toReadable(item.data.size)) }}</td>
      </tr>
    </tbody>
  </v-table>
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
</style>