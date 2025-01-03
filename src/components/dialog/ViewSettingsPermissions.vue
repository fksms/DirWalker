<script setup>

import { Command } from "@tauri-apps/plugin-shell";

function openPrivacy_FilesAndFolders() {
    new Command("open", "x-apple.systempreferences:com.apple.preference.security?Privacy_FilesAndFolders").execute();
}

function openPrivacy_AllFiles() {
    new Command("open", "x-apple.systempreferences:com.apple.preference.security?Privacy_AllFiles").execute();
}

</script>

<template>
    <h3>Files and Folders</h3>
    <p class="text-grey-lighten-2">If you want to allow access to the Downloads, Desktop, Documents folders, and Cloud
        storage folders, please click the button below to change the settings.</p>
    <v-btn flat class="text-capitalize mt-3" color="blue-grey-lighten-1" text="Open"
        @click="openPrivacy_FilesAndFolders()"></v-btn>

    <div class="py-6"></div>

    <h3>Full Disk Access</h3>
    <p class="text-grey-lighten-2">If you want to allow access to all data on this Mac, please click the button below to
        change the settings.</p>
    <v-btn flat class="text-capitalize mt-3" color="blue-grey-lighten-1" text="Open"
        @click="openPrivacy_AllFiles()"></v-btn>
</template>

<style></style>