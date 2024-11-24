<template>
  <div class="flex flex-wrap">
    <div class="flex box" v-for="i in items">
      <div class="icon-container">
        <span v-if="i.icon != undefined">{{ i.icon }}</span>
        <span v-else>{{ i.label.substring(0, 1) }}</span>
      </div>
      <span class="button-text">{{ i.label }}</span>
    </div>
    <div v-if="items.length == 0">
      <div class="no-items-container">No items to display.</div>
    </div>
  </div>
</template>
<style scoped>
.flex-wrap {
  flex-wrap: wrap;
}

.flex {
  display: flex;
}

.box {
  width: 64px;
  height: 64px;
  background-color: white;
  border-radius: 8px;
  padding: 6px 0 0;
  margin: 8px;
  margin-left: 0;
  flex-direction: column;
  overflow: hidden;
  align-items: center;
  white-space: nowrap;
  text-align: center;
  justify-content: flex-start;
}

.icon-container {
  background-color: #d9d9d9;
  width: 36px;
  height: 36px;
}

.button-text {
  margin-top: 4px;
}
</style>
<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { Ref, ref, watchEffect } from "vue";
const props = defineProps(["type"]);
let items: Ref<
  {
    name: string;
    icon: string;
    label: string;
  }[],
  | {
      name: string;
      icon: string;
      label: string;
    }[]
  | {
      name: string;
      icon: string;
      label: string;
    }[]
> = ref([{ name: "", icon: "", label: "" }]);
watchEffect(() => {
  items.value = [];
  if (props.type == "income") {
    invoke("get_income_accounts").then((res) => {
      let re = res as [{ name: string; icon: string }];
      items.value = re.map((it) => ({
        label: it.name.split("::")[it.name.split("::").length - 1],
        icon: it.icon,
        name: it.name,
      }));
    });
  } else if (props.type == "expense") {
    invoke("get_expenses_accounts").then((res) => {
      let re = res as [{ name: string; icon: string }];
      items.value = re.map((it) => ({
        label: it.name.split("::")[it.name.split("::").length - 1],
        icon: it.icon,
        name: it.name,
      }));
    });
  }
});
</script>
