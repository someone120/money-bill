import { createI18n } from 'vue-i18n'
import en from './assets/i18n/en.json'
import zh from './assets/i18n/zh.json'

// 定义语言包
const messages = {
  en,
  zh
}

export default createI18n({
  legacy: false, // 使用 composition API 模式
  locale: 'zh',  // 默认语言
  fallbackLocale: 'en', // 回退语言
  globalInjection: true, // 全局注入 $t 方法
  messages
})
