<script setup>

import { ref } from "vue";

import '@mdi/font/css/materialdesignicons.css'


// 描画用情報格納用
const ownColor = ref();
const ownName = ref();
const ownSize = ref();
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


// TB/GB/MB/KBに変換
function toReadable(size) {
  if (size >= 1e12) {
    return [(size / 1e12).toFixed(1), "TB"];
  }
  else if (size >= 1e9) {
    return [(size / 1e9).toFixed(1), "GB"];
  }
  else if (size >= 1e6) {
    return [(size / 1e6).toFixed(1), "MB"];
  }
  else if (size >= 1e3) {
    return [(size / 1e3).toFixed(1), "KB"];
  }
  else {
    return [size.toFixed(1), "B"];
  }
}


// 配列から文字列に変換
function array2String(array) {
  // デリミタを指定
  const delimiter = " "
  return array.join(delimiter);
}


// 外部から参照可能なプロパティを定義
defineExpose({
  generateDirectoryList,
  toReadable,
});


</script>

<template>
  <v-table class="bg-transparent text-white">
    <tbody>
      <tr v-if="ownColor && ownName && ownSize">
        <th width="10%"><v-icon :color="ownColor">mdi-circle</v-icon></th>
        <th width="65%" nowrap class="text-left">{{ getLastPath(ownName) }}</th>
        <th width="25%" nowrap class="text-right">{{ array2String(toReadable(ownSize)) }}</th>
      </tr>
    </tbody>
  </v-table>
  <v-table density="compact" class="bg-transparent text-white" hover height="400">
    <tbody>
      <tr v-for="item in children" :key="item.data.name" @click="">
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
}
</style>