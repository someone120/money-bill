<template>
  <div class="flex flex-wrap flex-column">
    <BillTab @change-type="changeType" />

    <AddBillItems :type="a" />
  </div>
</template>
<style scoped>
.flex-wrap {
  flex-wrap: wrap;
}
.flex-column {
  flex-direction: column;
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
import { ref } from "vue";
import AddBillItems from "./AddBillItems.vue";
import BillTab from "./BillTab.vue";

let a = ref("income");

function changeType(type) {
  a.value = type;

  console.log(a.value);
}
</script>
