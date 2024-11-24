<template>
  <div class="tabs">
    <button
      @click="changeType('income')"
      :class="{ active: selectedTab === 'income' }"
    >
      Tab 1
    </button>
    <button
      @click="changeType('expense')"
      :class="{ active: selectedTab === 'expense' }"
    >
      Tab 2
    </button>
    <button
      @click="selectedTab = 'tab3'"
      :class="{ active: selectedTab === 'tab3' }"
    >
      Tab 3
    </button>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";

const emit = defineEmits(["changeType"]);
let selectedTab = ref("tab1");
function changeType(type: string) {
  selectedTab.value = type;
  emit("changeType", selectedTab.value);
}
</script>

<style scoped>
.tabs {
  display: flex;
}

button {
  padding: 10px;
  margin-right: 10px;
}

button.active {
  background-color: #f0f0f0;
}
</style>
