<template>
  <div class="flex flex-row mb-2 w-full">
    <div class="flex flex-col flex-1 border rounded-lg mr-2 bg-white h-20 p-2">
      <div class="flex items-center">
        <img src="/svg/income.svg" class="w-6 h-6 mr-1" />
        <p class="text-gray-400 text-sm">{{ getString("income") }}</p>
      </div>
      <div class="flex items-center mt-2">
        <p class="ml-4 text-sm">¥</p>
        <p class="ml-4 text-sm">{{ income.toFixed(2) }}</p>
      </div>
    </div>
    <div class="flex flex-col flex-1 border rounded-lg mr-2 bg-white h-20 p-2">
      <div class="flex items-center">
        <img src="/svg/expenses.svg" class="w-6 h-6 mr-1" />
        <p class="text-gray-400 text-sm">{{ getString("expenses") }}</p>
      </div>
      <div class="flex items-center mt-2">
        <p class="ml-4 text-sm">¥</p>
        <p class="ml-4 text-sm">{{ expenses.toFixed(2) }}</p>
      </div>
    </div>
    <div class="flex flex-col flex-1 border rounded-lg bg-white h-20 p-2">
      <div class="flex items-center">
        <img src="/svg/balance.svg" class="w-6 h-6 mr-1" />
        <p class="text-gray-400 text-sm">{{ getString("balance") }}</p>
      </div>
      <div class="flex items-center mt-2">
        <p class="ml-4 text-sm">¥</p>
        <p class="ml-4 text-sm">{{ (income - expenses).toFixed(2) }}</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { i18n } from "../../i18n";
import { invoke } from "@tauri-apps/api/core";

const income = ref(0);
const expenses = ref(0);
let i18 = i18n.getInstace("zh_CN");
function getString(key: string): string {
  return i18.getString(key);
}

import { getCurrentInstance } from "vue";
// function ch(){
//   i18.changeLocale('en_US')
//   instance!.proxy!.$forceUpdate();
// }
invoke("get_income_expenses").then((i) => {
  income.value = (i as number[])[0];
  expenses.value = (i as number[])[1];
  console.log(i);
});
</script>
