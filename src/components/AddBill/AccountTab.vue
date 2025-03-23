<template>
  <div class="flex">
    <button
      @click="changeType('income')"
      :class="['px-4 py-2 mr-2', { 'bg-gray-100': selectedTab === 'income' }]"
    >
      {{ getString("income") }}
    </button>
    <button
      @click="changeType('expenses')"
      :class="['px-4 py-2 mr-2', { 'bg-gray-100': selectedTab === 'expense' }]"
    >
      {{ getString("expenses") }}
    </button>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { i18n } from "../../i18n";
let i18 = i18n.getInstace("zh_CN");
function getString(key: string): string {
  return i18.getString(key);
}
const emit = defineEmits(["changeType"]);
let selectedTab = ref("income");
function changeType(type: string) {
  selectedTab.value = type;
  emit("changeType", selectedTab.value);
}
</script>

<style scoped></style>
