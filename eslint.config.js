import js from '@eslint/js';
import prettier from 'eslint-config-prettier';
import simpleImportSort from 'eslint-plugin-simple-import-sort';
import pluginVue from 'eslint-plugin-vue';
import globals from 'globals';

export default [
    // add more generic rulesets here, such as:
    js.configs.recommended,
    ...pluginVue.configs['flat/recommended'],
    // ...pluginVue.configs['flat/vue2-recommended'], // Use this if you are using Vue.js 2.x.
    prettier,
    {
        plugins: {
            'simple-import-sort': simpleImportSort,
        },
        rules: {
            // 未使用変数の検出
            'no-unused-vars': [
                'warn',
                {
                    argsIgnorePattern: '^_',
                    varsIgnorePattern: '^_',
                    caughtErrorsIgnorePattern: '^_',
                    destructuredArrayIgnorePattern: '^_',
                },
            ],
            // 単語1つのコンポーネント名を許可
            'vue/multi-word-component-names': 'off',
            // propsの直接変更禁止
            'vue/no-mutating-props': 'error',
            // v-htmlの使用警告（XSS対策）
            'vue/no-v-html': 'warn',
            // propsに型指定を必須化
            'vue/require-prop-types': 'warn',
            // propsのデフォルト値必須化
            'vue/require-default-prop': 'warn',
            // 未使用コンポーネントの検出
            'vue/no-unused-components': 'warn',
            // 未使用変数の検出（Vueテンプレート内）
            'vue/no-unused-vars': 'warn',
            // テンプレート内のスコープシャドウ禁止
            'vue/no-template-shadow': 'warn',
            // 属性名はケバブケース推奨
            'vue/attribute-hyphenation': ['warn', 'always'],
            // テンプレート内のコンポーネント名のケース統一
            'vue/component-name-in-template-casing': ['warn', 'PascalCase'],
            // emitsオプションの明示化
            'vue/require-explicit-emits': 'warn',
            // 非推奨修飾子の警告
            'vue/no-deprecated-v-on-native-modifier': 'error',

            // --- 厳しめルール追加 ---
            // v-forでtemplate要素にkey属性必須
            'vue/no-v-for-template-key': 'error',
            // 属性の重複禁止
            'vue/no-duplicate-attributes': 'error',
            // 予約語のコンポーネント名禁止
            'vue/no-reserved-component-names': 'error',
            // computed内の副作用禁止
            'vue/no-side-effects-in-computed-properties': 'error',
            // propsのデフォルト値の型チェック
            'vue/require-valid-default-prop': 'error',
            // テンプレート構文エラー検出
            'vue/no-parsing-error': 'error',
            // 不要なtemplate属性禁止
            'vue/no-useless-template-attributes': 'error',
            // 未使用のprops/data/computed/methods検出
            'vue/no-unused-properties': 'error',
            // 空の<template>/<script>/<style>禁止
            'vue/no-empty-component-block': 'error',
            // 未定義のプロパティ参照禁止
            'vue/no-undef-properties': 'error',
            // await後のライフサイクルフック禁止
            'vue/no-lifecycle-after-await': 'error',
            // refを演算子のオペランドに使用禁止
            'vue/no-ref-as-operand': 'error',
            // script setup内でexport禁止（型定義以外）
            'vue/no-export-in-script-setup': 'error',
            // template要素へのkey属性禁止（v-for以外）
            'vue/no-template-key': 'error',
            // 不要なv-bind禁止
            'vue/no-useless-v-bind': 'error',
            // コンポーネントへのv-text/v-html禁止
            'vue/no-v-text-v-html-on-component': 'error',
            // script setupで直接export必須
            'vue/require-direct-export': 'error',
            // nameプロパティ必須化
            'vue/require-name-property': 'error',
            // render関数でreturn必須
            'vue/require-render-return': 'error',
            // slotsは関数として必須
            'vue/require-slots-as-functions': 'error',
            // オブジェクト型propsは型指定必須
            'vue/require-typed-object-prop': 'error',
            // defineEmitsの妥当性チェック
            'vue/valid-define-emits': 'error',
            // definePropsの妥当性チェック
            'vue/valid-define-props': 'error',
            // watchでアロー関数禁止（thisの誤用防止）
            'vue/no-arrow-functions-in-watch': 'error',
            // computedでasync関数禁止
            'vue/no-async-in-computed-properties': 'error',
            // 自閉タグに子要素禁止
            'vue/no-child-content': 'error',
            // テンプレート内の常にtrue/falseな条件禁止
            'vue/no-constant-condition': 'error',
            // 空の分割代入禁止
            'vue/no-empty-pattern': 'error',
            // 使用禁止構文の指定（例: ForInStatement禁止）
            'vue/no-restricted-syntax': ['error', 'ForInStatement'],
            // 未使用ref検出
            'vue/no-unused-refs': 'error',
            // v-ifとv-forの併用禁止
            'vue/no-use-v-if-with-v-for': 'error',
            // is属性必須化
            'vue/require-component-is': 'error',
            // v-slot構文の妥当性チェック
            'vue/valid-v-slot': 'error',
            // --- ここまで ---

            // --- インポート整理 ---
            // import文のソートを強制
            'simple-import-sort/imports': 'error',
            // export文のソートを強制
            'simple-import-sort/exports': 'error',
        },
        languageOptions: {
            sourceType: 'module',
            globals: {
                ...globals.browser,
            },
        },
    },
];
