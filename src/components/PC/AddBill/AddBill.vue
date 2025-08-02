<template>
  <div class="max-w-2xl mx-auto p-6 bg-white rounded-lg shadow-sm">
    <!-- 基本信息 -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-2">日期</label>
        <VueDatePicker
          v-model="date"
          placeholder="请选择日期"
          locale="zh-CN"
          class="w-full"
          :enable-time-picker="false"
          :clearable="false"
          :calendar-cell-class-name="false"
          :input-class-name="'dp-custom-input'"
        />
      </div>
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-2">备注</label>
        <input
          type="text"
          v-model="extra"
          placeholder="请输入备注"
          class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
        />
      </div>
    </div>

    <!-- 账户和金额列表 -->
    <div class="space-y-4 mb-6">
      <div
        v-for="(account, index) in accountList"
        :key="index"
        class="grid grid-cols-1 md:grid-cols-2 gap-4 items-end"
      >
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">账户 {{ index + 1 }}</label>
          <AddBillAccount
            :id="'' + index"
            :displayAccount="account"
            @changeAccount="changeAccount"
          />
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">金额</label>
          <div class="flex gap-2">
            <input
              type="number"
              v-model="amounts[index]"
              placeholder="请输入金额"
              class="flex-1 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
            />
            <button
              v-if="index > 0"
              @click="deleteAccount(index)"
              class="w-10 h-10 flex items-center justify-center text-red-600 hover:bg-red-50 rounded-md transition-colors flex-shrink-0"
              title="删除账户"
            >
              ×
            </button>
            <div
              v-else
              class="w-10 h-10 flex-shrink-0"
            ></div>
          </div>
        </div>
      </div>
    </div>

    <!-- 操作按钮 -->
    <div class="flex justify-end">
      <button
        @click="addTransaction"
        class="px-6 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 transition-colors"
      >
        添加交易
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

const accountList: Ref<AccountItem[]> = ref([
  {
    name: "选择账户",
    icon: "/svg/wallet.svg",
  },
]);
const amounts = ref([]);
const date = ref(new Date());
const extra = ref("");
const currency = ref("");

const changeAccount = (account: AccountItem, index: string) => {
  const idx = parseInt(index);
  accountList.value[idx] = account;

  // 如果是最后一个账户被选择了，自动添加新的空账户
  if (idx === accountList.value.length - 1) {
    accountList.value.push({
      name: "选择账户",
      icon: "/svg/wallet.svg",
    });
  }
};

const deleteAccount = (index: number) => {
  accountList.value.splice(index, 1);
  amounts.value.splice(index, 1);
};

const addTransaction = () => {
  // 过滤掉未选择的账户
  const validAccounts = accountList.value
    .map((account, index) => ({
      account: account.name,
      amount: parseFloat(amounts.value[index] || "0"),
    }))
    .filter(item => item.account !== "选择账户" && !isNaN(item.amount) && item.amount !== 0);

  if (validAccounts.length === 0) {
    alert("请至少选择一个账户并输入金额");
    return;
  }

  const data = {
    accountAmounts: validAccounts,
    date: Math.floor(date.value.getTime() / 1000),
    extra: extra.value,
    currency: currency.value,
  };

  console.log("添加交易:", data);

  invoke("add_bills", data)
    .then(() => {
      // 重置表单
      accountList.value = [{
        name: "选择账户",
        icon: "/svg/wallet.svg",
      }];
      amounts.value = [];
      extra.value = "";
      date.value = new Date();
    })
    .catch(error => {
      console.error("添加交易失败:", error);
      alert("添加交易失败，请重试");
    });
};
</script>

<style scoped>
/* VueDatePicker 样式调整 */
:deep(.dp__input) {
  padding: 0.5rem 0.75rem !important;
  border: 1px solid #d1d5db !important;
  border-radius: 0.375rem !important;
  transition: all 0.15s ease-in-out !important;
  background-image: none !important;
}

:deep(.dp__input:focus) {
  outline: none !important;
  border-color: #3b82f6 !important;
  box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.5) !important;
}

:deep(.dp__input_wrap) {
  border: none !important;
}

:deep(.dp__main) {
  border: none !important;
}

/* 隐藏所有图标 */
:deep(.dp__input_icon),
:deep(.dp__clear_icon),
:deep(.dp__calendar_icon) {
  display: none !important;
}

/* 确保输入框右侧没有额外的空间 */
:deep(.dp__input_icons) {
  display: none !important;
}
</style>
