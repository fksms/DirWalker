<script setup>

import { ref, watch } from "vue";



function generateDirectoryList(data) {
  console.log(data);
  const size = toReadable(data.value)
  console.log(size[0] + " " + size[1]);
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


// 外部から参照可能なプロパティを定義
defineExpose({
  generateDirectoryList,
  toReadable,
});


const desserts = [
  {
    name: 'Frozen Yogurt',
    calories: 159,
  },
  {
    name: 'Ice cream sandwich',
    calories: 237,
  },
  {
    name: 'Eclair',
    calories: 262,
  },
  {
    name: 'Cupcake',
    calories: 305,
  },
  {
    name: 'Gingerbread',
    calories: 356,
  },
  {
    name: 'Jelly bean',
    calories: 375,
  },
  {
    name: 'Lollipop',
    calories: 392,
  },
  {
    name: 'Honeycomb',
    calories: 408,
  },
  {
    name: 'Donut',
    calories: 452,
  },
  {
    name: 'KitKat',
    calories: 518,
  },
]

</script>

<template>
  <v-data-table-virtual :items="desserts" density="compact" height="400"></v-data-table-virtual>
</template>