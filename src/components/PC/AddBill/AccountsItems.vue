<template>
  <div class="p-2">
    <!-- 账户网格布局 -->
    <div class="grid grid-cols-4 gap-3" v-if="items.length > 0">
      <div
        v-for="(item, index) in items"
        :key="item.name"
        class="flex flex-col items-center p-3 rounded-lg cursor-pointer transition-all duration-200 hover:bg-gray-50 hover:shadow-md group border border-transparent hover:border-gray-200"
        @click="emitSelect(item)"
        :style="{ animationDelay: `${index * 50}ms` }"
      >
        <!-- 账户图标容器 -->
        <div class="relative mb-2">
          <div
            class="w-10 h-10 rounded-lg flex items-center justify-center transition-all duration-200 group-hover:scale-110"
            :class="getIconBackgroundClass(item)"
          >
            <!-- 自定义图标 -->
            <div
              v-if="item.icon && item.icon !== ''"
              class="text-lg transition-transform duration-200 group-hover:scale-110"
            >
              {{ item.icon }}
            </div>
            <!-- 文字图标 -->
            <div
              v-else
              class="w-8 h-8 rounded-md bg-gradient-to-br from-blue-400 to-blue-600 flex items-center justify-center text-white font-semibold text-sm transition-transform duration-200 group-hover:scale-110"
            >
              {{ item.name.split("::")[item.name.split("::").length - 1].substring(0, 1) }}
            </div>
          </div>

          <!-- 悬停时的光环效果 -->
          <div class="absolute inset-0 rounded-lg bg-blue-400 opacity-0 group-hover:opacity-20 transition-opacity duration-200 -z-10 scale-125"></div>
        </div>

        <!-- 账户名称 -->
        <span
          class="text-xs text-gray-700 text-center leading-tight group-hover:text-gray-900 transition-colors duration-200 font-medium truncate w-full"
          :title="item.name.split('::')[item.name.split('::').length - 1]"
        >
          {{ item.name.split("::")[item.name.split("::").length - 1] }}
        </span>

        <!-- 选中指示器 -->
        <div class="absolute -top-1 -right-1 w-3 h-3 bg-blue-500 rounded-full opacity-0 group-hover:opacity-100 transition-opacity duration-200 flex items-center justify-center">
          <svg class="w-2 h-2 text-white" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"></path>
          </svg>
        </div>
      </div>
    </div>

    <!-- 空状态 -->
    <div v-else class="flex flex-col items-center justify-center py-12 text-gray-500">
      <div class="w-16 h-16 rounded-full bg-gray-100 flex items-center justify-center mb-4">
        <svg class="w-8 h-8 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"></path>
        </svg>
      </div>
      <p class="text-sm font-medium text-gray-600 mb-1">暂无账户</p>
      <p class="text-xs text-gray-400 text-center">请先添加相关类型的账户</p>
    </div>

    <!-- 加载状态 -->
    <div v-if="isLoading" class="flex items-center justify-center py-8">
      <div class="flex space-x-2">
        <div class="w-2 h-2 bg-blue-400 rounded-full animate-bounce"></div>
        <div class="w-2 h-2 bg-blue-400 rounded-full animate-bounce" style="animation-delay: 0.1s"></div>
        <div class="w-2 h-2 bg-blue-400 rounded-full animate-bounce" style="animation-delay: 0.2s"></div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core"
import { Ref, ref, watchEffect } from "vue"
import type { AccountItem } from "./types"

const props = defineProps(["type"])
const emit = defineEmits<{
  (e: 'select', account: AccountItem): void
}>()

const items: Ref<AccountItem[]> = ref([])
const isLoading = ref(false)

const emitSelect = (account: AccountItem) => {
  emit('select', account)
}

// 根据账户类型生成不同的背景颜色
const getIconBackgroundClass = (item: AccountItem) => {
  if (item.icon && item.icon !== '') {
    return 'bg-gradient-to-br from-gray-50 to-gray-100 group-hover:from-blue-50 group-hover:to-blue-100'
  }
  return ''
}

watchEffect(() => {
  items.value = []
  isLoading.value = true

  if (props.type === "income") {
    invoke("get_income_accounts").then((res) => {
      const re = res as [{ name: string; icon: string }]
      items.value = re.map((it) => ({
        icon: it.icon,
        name: it.name,
      }))
      isLoading.value = false
    }).catch(() => {
      isLoading.value = false
    })
  } else if (props.type === "expenses") {
    invoke("get_expenses_accounts").then((res) => {
      const re = res as [{ name: string; icon: string }]
      items.value = re.map((it) => ({
        icon: it.icon,
        name: it.name,
      }))
      isLoading.value = false
    }).catch(() => {
      isLoading.value = false
    })
  } else {
    isLoading.value = false
  }
})
</script>

<style scoped>
/* 账户项目入场动画 */
@keyframes fadeInUp {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.grid > div {
  animation: fadeInUp 0.3s ease-out forwards;
  opacity: 0;
}

/* 悬停时的微妙阴影效果 */
.group:hover {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
}

/* 点击反馈 */
.group:active {
  transform: scale(0.98);
}

/* 确保文字不会溢出 */
.truncate {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* 自定义滚动条（如果需要） */
.overflow-y-auto::-webkit-scrollbar {
  width: 4px;
}

.overflow-y-auto::-webkit-scrollbar-track {
  background: transparent;
}

.overflow-y-auto::-webkit-scrollbar-thumb {
  background: #d1d5db;
  border-radius: 2px;
}

.overflow-y-auto::-webkit-scrollbar-thumb:hover {
  background: #9ca3af;
}

/* 响应式布局调整 */
@media (max-width: 640px) {
  .grid-cols-4 {
    grid-template-columns: repeat(3, minmax(0, 1fr));
  }
}

@media (max-width: 480px) {
  .grid-cols-4 {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }
}
</style>
