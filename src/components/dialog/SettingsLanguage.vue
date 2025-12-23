<script setup>
import { computed } from 'vue';

import i18n from '../../lib/i18n';

// 利用可能な言語
const currentLocale = i18n.global.availableLocales;

// 現在の言語に基づいて選択肢を取得
const localizedItems = computed(() => {
    return currentLocale.map((locale) => ({
        title: i18n.global.t(`language.${locale}`),
        value: locale,
    }));
});
</script>

<template>
    <h3>{{ $t('language.language_setting') }}</h3>
    <div class="py-2"></div>
    <v-select v-model="$i18n.locale" :items="localizedItems" item-title="title" item-value="value" density="compact"></v-select>
</template>
