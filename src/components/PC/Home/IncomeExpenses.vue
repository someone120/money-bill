<template>
  <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6 w-full">
    <!-- Income -->
    <div class="card flex flex-col justify-between p-4 border-t-4 !border-t-emerald-500">
      <div class="flex items-center gap-2 mb-2">
        <div class="p-1.5 rounded-full bg-emerald-100 dark:bg-emerald-900/30">
           <v-icon icon="mdi-arrow-down-bold" size="small" color="success" class="text-emerald-600"></v-icon>
        </div>
        <span class="text-sm font-medium text-muted-foreground">{{ $t("home.income") }}</span>
      </div>
      <p class="text-3xl font-heading font-bold text-emerald-600 truncate ml-1 pb-1">
        {{ income.toFixed(2) }}
      </p>
    </div>

    <!-- Expenses -->
    <div class="card flex flex-col justify-between p-4 border-t-4 !border-t-rose-500">
      <div class="flex items-center gap-2 mb-2">
        <div class="p-1.5 rounded-full bg-rose-100 dark:bg-rose-900/30">
           <v-icon icon="mdi-arrow-up-bold" size="small" color="error" class="text-rose-600"></v-icon>
        </div>
        <span class="text-sm font-medium text-muted-foreground">{{ $t("expenses") }}</span>
      </div>
      <p class="text-3xl font-heading font-bold text-rose-600 truncate ml-1 pb-1">
        {{ expenses.toFixed(2) }}
      </p>
    </div>

    <!-- Balance -->
    <div class="card flex flex-col justify-between p-4 border-t-4 !border-t-blue-500">
      <div class="flex items-center gap-2 mb-2">
        <div class="p-1.5 rounded-full bg-blue-100 dark:bg-blue-900/30">
           <v-icon icon="mdi-wallet" size="small" color="primary" class="text-blue-600"></v-icon>
        </div>
        <span class="text-sm font-medium text-muted-foreground">{{ $t("balance") }}</span>
      </div>
      <p class="text-3xl font-heading font-bold text-foreground truncate ml-1 pb-1">
        {{ (income - expenses).toFixed(2) }}
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const income = ref(0);
const expenses = ref(0);

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
