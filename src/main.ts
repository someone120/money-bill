import { createApp } from "vue";
import App from "./App.vue";
import router from "./router.ts";
import "./index.css";
import i18n from "./i18n"; // 引入 i18n 实例

// Vuetify
import 'vuetify/styles'
import { createVuetify } from 'vuetify'
import { aliases, mdi } from 'vuetify/iconsets/mdi'
import '@mdi/font/css/materialdesignicons.css'

const vuetify = createVuetify({
  icons: {
    defaultSet: 'mdi',
    aliases,
    sets: {
      mdi,
    },
  },
})

// const { t } = useI18n(); // 获取全局的 $t 方法
const app = createApp(App).use(router).use(i18n).use(vuetify);

// app.config.globalProperties.$t = t;

app.mount("#app");
