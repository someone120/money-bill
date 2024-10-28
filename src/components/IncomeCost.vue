<template>
  <div class="flex summary-bar" style="margin-bottom: 8px">
    <div class="flex flex1 border mgr8 flex-column box">
      <div
        class="flex flex1"
        style="margin: 5px; margin-bottom: 0; align-self: flex-start"
      >
        <img src="./../assets/svg/income.svg" class="icon" />
        <p style="margin: 4px; margin-bottom: 0" class="hint-color">
          {{ getString("income") }}
        </p>
      </div>
      <div class="flex flex1" style="margin-bottom: 8px">
        <p style="margin: 0; margin-left: 15px">¥</p>
        <p style="margin: 0; margin-left: 16px">{{ income.toFixed(2) }}</p>
      </div>
    </div>
    <div class="flex flex1 border mgr8 flex-column box">
      <div
        class="flex flex1"
        style="margin: 5px; margin-bottom: 0; align-self: flex-start"
      >
        <img src="./../assets/svg/expenses.svg" class="icon" />
        <p style="margin: 4px; margin-bottom: 0" class="hint-color">
          {{ getString("expenses") }}
        </p>
      </div>
      <div class="flex flex1" style="margin-bottom: 8px">
        <p style="margin: 0; margin-left: 15px">¥</p>
        <p style="margin: 0; margin-left: 16px">{{ expenses.toFixed(2) }}</p>
      </div>
    </div>
    <div class="flex flex1 border flex-column box">
      <div
        class="flex flex1"
        style="margin: 5px; margin-bottom: 0; align-self: flex-start"
      >
        <img src="./../assets/svg/balance.svg" class="icon" />
        <p style="margin: 4px; margin-bottom: 0" class="hint-color">
          {{ getString("balance") }}
        </p>
      </div>
      <div class="flex flex1" style="margin-bottom: 8px">
        <p style="margin: 0; margin-left: 15px">¥</p>
        <p style="margin: 0; margin-left: 16px">
          {{ (income - expenses).toFixed(2) }}
        </p>
      </div>
    </div>
  </div>
</template>

<style scoped>
.box {
  height: 70px;
  background-color: white;
}
p {
  text-align: left;
  white-space: nowrap;
  overflow: hidden;
}

.hint-color {
  color: #c0c0c0;
}

.icon {
  margin: 4px;
  margin-bottom: 0;
  width: 25px;
  height: 25px;
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
  flex-wrap: nowrap;
}

.flex {
  display: flex;
}
</style>

<script setup lang="ts">
import { ref } from "vue";
import { i18n } from "../i18n";
import { invoke } from "@tauri-apps/api/core";

const income = ref(0);
const expenses = ref(0);
let i18 = i18n.getInstace("zh_CN");
function getString(key: string): string {
  return i18.getString(key);
}

// import {getCurrentInstance} from 'vue';
// function ch(){
//   i18.changeLocale('en_US')
//   instance!.proxy!.$forceUpdate();
// }
invoke("get_income_expenses").then((i) => {
  income.value = i as number[][0];
  expenses.value = i as number[][0];
  console.log(i);
});
</script>
