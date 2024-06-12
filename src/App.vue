<script setup>

import { ref } from "vue";

import ViewHeader from "./components/ViewHeader.vue";
import ViewSunburstChart from "./components/ViewSunburstChart.vue";
import ViewDirectoryFileList from "./components/ViewDirectoryFileList.vue";

// コンポーネントの参照を作成
const viewHeader = ref();
const viewSunburstChart = ref();
const viewDirectoryFileList = ref();

</script>

<template>
  <div style="background-color: #37474F; height: 100vh;">

    <ViewHeader ref="viewHeader" :viewSunburstChart="viewSunburstChart"></ViewHeader>

    <v-divider class="border-opacity-25" color="blue-grey-lighten-3"></v-divider>

    <v-container fluid>
      <v-row>
        <v-col>
          <ViewSunburstChart ref="viewSunburstChart" :viewDirectoryFileList="viewDirectoryFileList"></ViewSunburstChart>
        </v-col>
        <v-col>
          <ViewDirectoryFileList ref="viewDirectoryFileList"></ViewDirectoryFileList>
        </v-col>
      </v-row>
    </v-container>

  </div>
</template>

<style></style>
