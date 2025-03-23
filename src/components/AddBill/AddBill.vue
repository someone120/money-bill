<template>
  <div class="flex flex-col">
    <div class="flex flex-row m-2 h-[38px]">
      <VueDatePicker
        v-model="date"
        placeholder="请选择日期"
        locale="zh-CN"
        class="mr-4"
      ></VueDatePicker>
      <input
        type="text"
        class="flex-grow rounded border border-gray-300 pl-3"
        placeholder="备注"
        v-model="extra"
      />
    </div>
    <div
      class="flex flex-row m-2"
      v-for="index in accountList.length"
      :key="index"
    >
      <AddBillAccount
        class="flex-grow h-12"
        @changeAccount="changeAccount"
        :id="'' + (index - 1)"
        :displayAccount="accountList[index - 1]"
      />
      <input
        type="number"
        :id="'Amount_' + (index - 1)"
        class="ml-4 h-12 rounded pl-3 border border-gray-300"
        placeholder="金额"
        v-model="amounts[index - 1]"
      />
      <button
        @click="deleteAccount(index - 1)"
        class="ml-4 h-12 rounded p-2 border border-gray-300"
        v-if="index != 1"
      >
        <img src="/svg/delete.svg" alt="删除" class="h-full w-full" />
      </button>
    </div>
    <div>
      <button
        @click="addTransaction"
        class="m-2 h-12 rounded p-3 border border-gray-300"
      >
        确认
      </button>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { Ref, ref } from "vue";
import VueDatePicker from "@vuepic/vue-datepicker";
import "@vuepic/vue-datepicker/dist/main.css";
import AddBillAccount from "./AddBillAccount.vue";
import { AccountItem } from "./types";
import { invoke } from "@tauri-apps/api/core";

let accountList: Ref<AccountItem[]> = ref([
  {
    name: "选择账户",
    icon: "/svg/wallet.svg",
  },
]);
const amounts = ref([]);
const date = ref(new Date());
const extra = ref("");
const changeAccount = (account: AccountItem, index: string) => {
  let index1 = parseInt(index);

  if (accountList.value[index1 + 1] === undefined) {
    accountList.value.push({
      name: "选择账户",
      icon: "/svg/wallet.svg",
    });
  }
  console.log(accountList);
  accountList.value[index1] = account;
};

const deleteAccount = (index: number) => {
  console.log(index);

  accountList.value.splice(index, 1);
  console.log(accountList.value);
};

const addTransaction = () => {
  let accounts = accountList.value.map((it) => it.name);
  accounts.pop();
  let amounts1 = amounts.value.map((it) => parseFloat(it));

  let data = accounts.map(function (e, i) {
    return { account: e, amount: amounts1[i] };
  });
  console.log({
    accountAmounts: data,
    date: parseInt(''+new Date().getTime()/1000),
    extra: extra,
  });
  invoke("add_bills", {
    accountAmounts: data,
    date: parseInt(''+new Date().getTime()/1000),
    extra: extra.value,
  });
};
</script>
