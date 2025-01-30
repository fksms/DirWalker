import { createI18n } from "vue-i18n";

import i18n_ja from "./i18n/ja";
import i18n_en from "./i18n/en";

const userLocale = navigator.language || navigator.userLanguage;

const i18n = createI18n({
    // For composition API
    legacy: false,
    // デフォルトの言語を日本語に固定する
    locale: userLocale.includes("ja") ? "ja" : "en",
    // 言語の定義
    messages: {
        ja: i18n_ja,
        en: i18n_en,
    },
});

export default i18n;
