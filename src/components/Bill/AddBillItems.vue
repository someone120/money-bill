<template>
  <div class="flex flex-wrap">
    <div
      class="flex flex-col items-center justify-start w-16 h-16 bg-white rounded-lg p-0 pt-1.5 m-2 ml-0 overflow-hidden whitespace-nowrap text-center"
      v-for="i in items"
      :key="i.name"
    >
      <div class="bg-gray-300 w-9 h-9 rounded-lg flex items-center justify-center">
        <span v-if="i.icon">{{ i.icon }}</span>
        <span v-else>{{ i.label.substring(0, 1) }}</span>
      </div>
      <span class="mt-1">{{ i.label }}</span>
    </div>
    <div v-if="items.length === 0" class="w-full">
      <div class="text-center">No items to display.</div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { Ref, ref, watchEffect } from "vue";

const props = defineProps(["type"]);
const items: Ref<
  {
    name: string;
    icon: string;
    label: string;
  }[]
> = ref([]);
watchEffect(() => {
  items.value = [];
  if (props.type === "income") {
    invoke("get_income_accounts").then((res) => {
      const re = res as [{ name: string; icon: string }];
      items.value = re.map((it) => ({
        label: it.name.split("::")[it.name.split("::").length - 1],
        icon: it.icon,
        name: it.name,
      }));
    });
  } else if (props.type === "expense") {
    invoke("get_expenses_accounts").then((res) => {
      const re = res as [{ name: string; icon: string }];
      items.value = re.map((it) => ({
        label: it.name.split("::")[it.name.split("::").length - 1],
        icon: it.icon,
        name: it.name,
      }));
    });
  }
});
</script>
