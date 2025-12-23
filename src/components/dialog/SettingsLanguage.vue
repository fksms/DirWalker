<script setup>
// 選択肢を固定で定義
const localizedItems = [
    { title: '日本語', value: 'ja' },
    { title: 'English', value: 'en' },
];
</script>

<template>
    <h3>{{ $t('language.language_setting') }}</h3>
    <div class="py-2"></div>
    <v-select v-model="$i18n.locale" :items="localizedItems" item-title="title" item-value="value" density="compact"></v-select>
</template>
