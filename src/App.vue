<script setup>

import { ref } from "vue";

import ViewHeader from "./components/ViewHeader.vue";
import ViewSunburstChart from "./components/ViewSunburstChart.vue";
import ViewDirectoryFileList from "./components/ViewDirectoryFileList.vue";
import ViewBreadcrumbsList from "./components/ViewBreadcrumbsList.vue";


// コンポーネントの参照を作成
const viewHeader = ref();
const viewSunburstChart = ref();
const viewDirectoryFileList = ref();
const viewBreadcrumbsList = ref();

</script>

<template>
    <div class="bg-blue-grey-darken-3" style="height: 100vh;">

        <ViewHeader ref="viewHeader" :viewSunburstChart="viewSunburstChart"></ViewHeader>

        <v-divider class="border-opacity-25" color="blue-grey-lighten-3"></v-divider>

        <ViewBreadcrumbsList ref="viewBreadcrumbsList" :viewSunburstChart="viewSunburstChart"
            :viewDirectoryFileList="viewDirectoryFileList"></ViewBreadcrumbsList>

        <v-container fluid class="pt-0 pb-0">
            <v-row>
                <v-col align-self="start" class="pt-10">
                    <ViewSunburstChart ref="viewSunburstChart" :viewDirectoryFileList="viewDirectoryFileList"
                        :viewBreadcrumbsList="viewBreadcrumbsList">
                    </ViewSunburstChart>
                </v-col>
                <v-col align-self="start">
                    <ViewDirectoryFileList ref="viewDirectoryFileList" :viewSunburstChart="viewSunburstChart">
                    </ViewDirectoryFileList>
                </v-col>
            </v-row>
        </v-container>

    </div>
</template>

<style>
:root {
    /* フォントの指定 */
    font-family: Avenir, YuGothic, 'Yu Gothic', Futura, 'Century Gothic', 'Hiragino Kaku Gothic ProN', Meiryo, sans-serif;

    /* テキストの選択不可 */
    user-select: none;
    -webkit-user-select: none;
    -ms-user-select: none;
    -moz-user-select: none;
    -khtml-user-select: none;
    -webkit-user-drag: none;
    -khtml-user-drag: none;
}
</style>
