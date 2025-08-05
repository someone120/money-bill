<template>
  <div class="relative">
    <!-- 选择器按钮 -->
    <div
      @click="toggleDropdown"
      class="w-full px-3 py-2 border border-gray-300 rounded-md cursor-pointer hover:border-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 bg-white flex items-center justify-between"
    >
      <div class="flex items-center">
        <div class="w-6 h-6 mr-2 rounded bg-gray-100 flex items-center justify-center text-xs font-medium text-gray-600">
            {{ displayAccount.name?.split("::").slice(-1)[0]?.substring(0, 1) || "?" }}
        </div>
        <span class="text-gray-700">
          {{ displayAccount.name.split("::").slice(-1)[0] || "选择账户" }}
        </span>
      </div>
      <svg
        class="w-4 h-4 text-gray-400 transition-transform"
        :class="{ 'rotate-180': isOpen }"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
      >
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
      </svg>
    </div>

    <!-- 下拉面板 -->
    <div
      v-if="isOpen"
      class="absolute top-full left-0 right-0 mt-1 bg-white border border-gray-300 rounded-md shadow-lg z-50"
    >
      <!-- 标签切换 -->
      <div class="flex border-b border-gray-200">
        <button
          @click="currentType = 'income'"
          :class="[
            'flex-1 px-4 py-2 text-sm font-medium',
            currentType === 'income'
              ? 'text-blue-600 border-b-2 border-blue-600 bg-blue-50'
              : 'text-gray-500 hover:text-gray-700'
          ]"
        >
          收入
        </button>
        <button
          @click="currentType = 'expenses'"
          :class="[
            'flex-1 px-4 py-2 text-sm font-medium',
            currentType === 'expenses'
              ? 'text-red-600 border-b-2 border-red-600 bg-red-50'
              : 'text-gray-500 hover:text-gray-700'
          ]"
        >
          支出
        </button>
      </div>

      <!-- 账户列表 -->
      <div class="max-h-48 overflow-y-auto p-2">
        <div v-if="items.length === 0" class="text-gray-500 text-sm text-center py-4">
          暂无账户
        </div>
        <div
          v-for="item in items"
          :key="item.name"
          @click="selectAccount(item)"
          class="flex items-center px-3 py-2 hover:bg-gray-50 rounded cursor-pointer"
        >
          <div class="w-6 h-6 mr-3 rounded bg-gray-100 flex items-center justify-center text-xs font-medium text-gray-600">
              {{ item.name.split("::").slice(-1)[0]?.substring(0, 1) || "?" }}
          </div>
          <span class="text-gray-700 text-sm">
            {{ item.name.split("::").slice(-1)[0] }}
          </span>
        </div>
      </div>
    </div>

    <!-- 遮罩层 -->
    <div
      v-if="isOpen"
      @click="closeDropdown"
      class="fixed inset-0 z-40"
    ></div>
  </div>
</template>

<script setup lang="ts">
import { ref, watchEffect, onMounted, onUnmounted } from "vue";
import type { AccountItem } from "./types";
import { invoke } from "@tauri-apps/api/core";

const emit = defineEmits(["changeAccount"]);
const props = defineProps<{
  id: string;
  displayAccount: AccountItem;
}>();

const isOpen = ref(false);
const currentType = ref("income");
const items = ref<AccountItem[]>([]);

const toggleDropdown = () => {
  isOpen.value = !isOpen.value;
};

const closeDropdown = () => {
  isOpen.value = false;
};

const selectAccount = (account: AccountItem) => {
  emit("changeAccount", account, props.id);
  closeDropdown();
};

// 获取账户列表
watchEffect(() => {
  if (!isOpen.value) return;

  items.value = [];
  const method = currentType.value === "income" ? "get_income_accounts" : "get_expenses_accounts";

  invoke(method).then((res) => {
    const accounts = res as [{ name: string; icon: string }];
    items.value = accounts.map((it) => ({
      icon: it.icon,
      name: it.name,
    }));
  }).catch(() => {
    items.value = [];
  });
});

// 键盘事件
const handleEscape = (event: KeyboardEvent) => {
  if (event.key === 'Escape' && isOpen.value) {
    closeDropdown();
  }
};

onMounted(() => {
  document.addEventListener('keydown', handleEscape);
});

onUnmounted(() => {
  document.removeEventListener('keydown', handleEscape);
});
</script>
