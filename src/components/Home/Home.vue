<script setup lang="ts">
import AddBill from '../Bill/AddBill.vue';
import AddBillItems from '../Bill/AddBillItems.vue';
import ExpensesHistogram from './ExpensesHistogram.vue';
import IncomeExpenses from './IncomeExpenses.vue';

</script>

<template>
  <div class="flex flex1">
    <!-- 主要内容 -->
    <div style="flex: 3" class="flex flex-column primary-content mg8">
      <!-- 收入/支出/结余 -->
      <IncomeExpenses class="desk-only" />
      <ExpensesHistogram/>
      <AddBillItems type="income"/>
    </div>
    <!-- 每日支出/收入 -->
    <div style="flex: 2" class="flex flex-column calendar"></div>
  </div>
</template>
<style scoped>
.flex {
  display: flex;
}
.mgr8 {
  margin-right: 8px;
}

.mg8 {
  margin: 8px;
}

.border {
  border-radius: 10px;
}

.flex-column {
  flex-direction: column;
}

.flex1 {
  flex: 1;
}
@media screen and (max-width: 700px) {
  .desk-only {
    display: none !important;
  }
}
</style>