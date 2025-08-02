<template>
  <div class="flex bg-gray-50 p-1 rounded-lg">
    <button
      @click="changeType('income')"
      :class="[
        'flex-1 px-4 py-2 text-sm font-medium rounded-md transition-all duration-200 relative overflow-hidden',
        selectedTab === 'income'
          ? 'bg-white text-blue-600 shadow-sm border border-blue-100'
          : 'text-gray-600 hover:text-gray-800 hover:bg-gray-100'
      ]"
    >
      <span class="relative z-10 flex items-center justify-center">
        <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
        </svg>
        {{ getString("income") }}
      </span>
      <div
        v-if="selectedTab === 'income'"
        class="absolute inset-0 bg-gradient-to-r from-blue-50 to-blue-100 opacity-50"
      ></div>
    </button>

    <button
      @click="changeType('expenses')"
      :class="[
        'flex-1 px-4 py-2 text-sm font-medium rounded-md transition-all duration-200 relative overflow-hidden ml-1',
        selectedTab === 'expenses'
          ? 'bg-white text-red-600 shadow-sm border border-red-100'
          : 'text-gray-600 hover:text-gray-800 hover:bg-gray-100'
      ]"
    >
      <span class="relative z-10 flex items-center justify-center">
        <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 12H4"></path>
        </svg>
        {{ getString("expenses") }}
      </span>
      <div
        v-if="selectedTab === 'expenses'"
        class="absolute inset-0 bg-gradient-to-r from-red-50 to-red-100 opacity-50"
      ></div>
    </button>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { i18n } from "../../../i18n";

const props = defineProps<{
  selectedTab?: string;
}>();

let i18 = i18n.getInstace("zh_CN");
function getString(key: string): string {
  return i18.getString(key);
}

const emit = defineEmits(["changeType"]);
let selectedTab = ref(props.selectedTab || "income");

function changeType(type: string) {
  selectedTab.value = type;
  emit("changeType", selectedTab.value);
}
</script>

<style scoped>
/* 按钮点击效果 */
button:active {
  transform: scale(0.98);
}

/* 聚焦效果 */
button:focus {
  outline: none;
  box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.5);
}

/* 确保图标和文字对齐 */
.flex.items-center.justify-center {
  gap: 0.5rem;
}
</style>
