<script setup lang="ts">
import AddBill from "../Bill/AddBill.vue";
import AddBillItems from "../Bill/AddBillItems.vue";
import ExpensesHistogram from "./ExpensesHistogram.vue";
import IncomeExpenses from "./IncomeExpenses.vue";
</script>

<template>
  <div class="flex flex-1 w-full">
    <!-- 主要内容 -->
    <div class="flex flex-col m-2 w-full">
      <!-- 收入/支出/结余 -->
      <IncomeExpenses class="hidden md:flex" />
      <ExpensesHistogram />
      <!-- <AddBillItems type="income" /> -->
    </div>
    <!-- 每日支出/收入 -->
    <div class="flex flex-col"></div>
  </div>
</template>
<style scoped></style>
