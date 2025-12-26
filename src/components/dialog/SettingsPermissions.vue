<script setup>
import { invoke } from '@tauri-apps/api/core';
import { Command } from '@tauri-apps/plugin-shell';
import { onMounted, ref } from 'vue';

import { detectOS } from '../../lib/detectOS';

// フルディスクアクセスの権限状況を保持
const fullDiskAccessPermission = ref(false);

// マウントされた後に行う処理
onMounted(async () => {
    // Macの場合
    if (detectOS() == 'Mac') {
        await invoke('check_full_disk_access_permission', {})
            // 成功した場合
            .then((success) => {
                //console.log(success);
                fullDiskAccessPermission.value = success;
            })
            // 失敗した場合
            .catch((failure) => {
                console.log(failure);
            });
    }
});

function openPrivacy_FilesAndFolders() {
    new Command('open', 'x-apple.systempreferences:com.apple.preference.security?Privacy_FilesAndFolders').execute();
}

function openPrivacy_AllFiles() {
    new Command('open', 'x-apple.systempreferences:com.apple.preference.security?Privacy_AllFiles').execute();
}
</script>

<template>
    <h3>{{ $t('permissions.files_and_folders') }}</h3>
    <p class="text-grey-lighten-2">{{ $t('permissions.files_and_folders_desc') }}</p>
    <v-btn flat class="text-capitalize mt-3" color="blue-grey-lighten-1" text="Open" @click="openPrivacy_FilesAndFolders()"></v-btn>

    <div class="py-6"></div>

    <h3>{{ $t('permissions.full_disk_access') }}</h3>
    <p class="text-grey-lighten-2">{{ $t('permissions.full_disk_access_desc') }}</p>
    <p v-if="fullDiskAccessPermission" class="mt-3">{{ $t('permissions.full_disk_access_is_already_granted') }}</p>
    <v-btn v-else flat class="text-capitalize mt-3" color="blue-grey-lighten-1" text="Open" @click="openPrivacy_AllFiles()"></v-btn>
</template>
