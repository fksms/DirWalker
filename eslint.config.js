import pluginVue from 'eslint-plugin-vue';
import prettier from 'eslint-config-prettier';
import globals from 'globals';
import js from '@eslint/js';

export default [
    // add more generic rulesets here, such as:
    js.configs.recommended,
    ...pluginVue.configs['flat/recommended'],
    // ...pluginVue.configs['flat/vue2-recommended'], // Use this if you are using Vue.js 2.x.
    prettier,
    {
        rules: {
            // override/add rules settings here, such as:
            'no-unused-vars': 'warn',
        },
        languageOptions: {
            sourceType: 'module',
            globals: {
                ...globals.browser,
            },
        },
    },
];
