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
    <h3>{{ $t("permissions.files_and_folders") }}</h3>
    <p class="text-grey-lighten-2">{{ $t("permissions.files_and_folders_desc") }}</p>
    <v-btn flat class="text-capitalize mt-3" color="blue-grey-lighten-1" text="Open"
        @click="openPrivacy_FilesAndFolders()"></v-btn>

    <div class="py-6"></div>

    <h3>{{ $t("permissions.full_disk_access") }}</h3>
    <p class="text-grey-lighten-2">{{ $t("permissions.full_disk_access_desc") }}</p>
    <v-btn flat class="text-capitalize mt-3" color="blue-grey-lighten-1" text="Open"
        @click="openPrivacy_AllFiles()"></v-btn>
</template>

<style></style>