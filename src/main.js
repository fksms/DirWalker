// Styles
import 'vuetify/styles';
import '@mdi/font/css/materialdesignicons.css';

import { createApp } from 'vue';
import { createVuetify } from 'vuetify';

import App from './App.vue';
import i18n from './components/i18n';

const vuetify = createVuetify();

const app = createApp(App);
app.use(vuetify);
app.use(i18n);
app.mount('#app');
