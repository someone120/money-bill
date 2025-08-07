import { createApp } from "vue";
import App from "./App.vue";
import router from "./router.ts";
import "./index.css";
import i18n from "./i18n"; // 引入 i18n 实例

// const { t } = useI18n(); // 获取全局的 $t 方法
const app = createApp(App).use(router).use(i18n);

// app.config.globalProperties.$t = t;

app.mount("#app");
