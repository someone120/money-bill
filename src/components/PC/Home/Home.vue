<script setup lang="ts">
import ExpensesHistogram from "./ExpensesHistogram.vue";
import IncomeExpenses from "./IncomeExpenses.vue";
</script>

<template>
  <div class="flex flex-col flex-1 w-full h-full bg-background p-4 md:p-6 overflow-y-auto">
    <!-- Main Content Container -->
    <div class="flex flex-col w-full max-w-7xl mx-auto space-y-6">
      <!-- Income/Expenses Summary -->
      <IncomeExpenses />
      
      <div class="w-full h-px bg-border/60 my-2"></div>

      <!-- Charts Section -->
      <div class="w-full">
        <h3 class="text-lg font-heading font-bold mb-4 text-foreground">{{ $t("home.statistics") }}</h3>
        <ExpensesHistogram />
      </div>

      <!-- <AddBillItems type="income" /> -->
    </div>
    
    <!-- Daily list placeholder (future) -->
    <div class="flex flex-col"></div>
  </div>
</template>
<style scoped></style>
