<template>
  <div class="flex flex-wrap flex-col mx-4">
    <BillTab @change-type="changeType" />

    <AddBillItems :type="a" />
  </div>
</template>
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
