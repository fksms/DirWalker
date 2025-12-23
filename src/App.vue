<script setup>
import { ref } from 'vue';

import Navbar from './components/Navbar.vue';
import SunburstChart from './components/SunburstChart.vue';
import DirectoryFileList from './components/DirectoryFileList.vue';
import BreadcrumbsList from './components/BreadcrumbsList.vue';

// コンポーネントの参照を作成
const navbar = ref();
const sunburstChart = ref();
const directoryFileList = ref();
const breadcrumbsList = ref();
</script>

<template>
    <div class="bg-blue-grey-darken-3" style="height: 100vh">
        <Navbar ref="navbar" :sunburst-chart="sunburstChart"></Navbar>

        <v-divider class="border-opacity-25" color="blue-grey-lighten-3"></v-divider>

        <BreadcrumbsList ref="breadcrumbsList" :sunburst-chart="sunburstChart" :directory-file-list="directoryFileList"></BreadcrumbsList>

        <v-container fluid class="py-0">
            <v-row>
                <v-col align-self="start" class="pt-10">
                    <SunburstChart ref="sunburstChart" :directory-file-list="directoryFileList" :breadcrumbs-list="breadcrumbsList"> </SunburstChart>
                </v-col>
                <v-col align-self="start">
                    <DirectoryFileList ref="directoryFileList" :sunburst-chart="sunburstChart"> </DirectoryFileList>
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
