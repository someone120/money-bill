<template>
  <v-container>
    <!-- Basic Info -->
    <v-row>
      <v-col cols="12" md="6">
        <label class="text-caption text-grey-darken-1 mb-1 d-block">{{ $t("addBill.dateLabel") }}</label>
        <v-menu
          v-model="dateMenu"
          :close-on-content-click="false"
          transition="scale-transition"
          min-width="auto"
        >
          <template v-slot:activator="{ props }">
            <v-text-field
              v-model="formattedDate"
              :placeholder="$t('addBill.datePlaceholder')"
              prepend-inner-icon="mdi-calendar"
              readonly
              v-bind="props"
              variant="outlined"
              density="comfortable"
              hide-details
            ></v-text-field>
          </template>
          <v-date-picker
            v-model="date"
            color="primary"
            @update:modelValue="dateMenu = false"
          ></v-date-picker>
        </v-menu>
      </v-col>
      <v-col cols="12" md="6">
        <label class="text-caption text-grey-darken-1 mb-1 d-block">{{ $t("addBill.remarkLabel") }}</label>
        <v-text-field
          v-model="extra"
          :placeholder="$t('addBill.remarkPlaceholder')"
          variant="outlined"
          density="comfortable"
          hide-details
        ></v-text-field>
      </v-col>
    </v-row>

    <v-divider class="my-4"></v-divider>

    <!-- Account List -->
    <div v-for="(account, index) in accountList" :key="index" class="mb-4">
      <v-row align="center">
        <v-col cols="12" md="6">
           <label class="text-caption text-grey-darken-1 mb-1 d-block">
             {{ $t("addBill.account") }} {{ index + 1 }}
           </label>
           <AddBillAccount
             :id="'' + index"
             :displayAccount="account"
             @changeAccount="changeAccount"
           />
        </v-col>
        <v-col cols="12" md="6">
           <label class="text-caption text-grey-darken-1 mb-1 d-block">{{ $t("addBill.amount") }}</label>
           <div class="d-flex align-center">
             <v-text-field
               v-model="amounts[index]"
               :placeholder="$t('addBill.amountPlaceholder')"
               type="number"
               variant="outlined"
               density="compact"
               hide-details
               prefix="¥"
               class="flex-grow-1"
             ></v-text-field>
             
             <v-btn
               v-if="index > 0"
               icon="mdi-delete"
               variant="text"
               color="error"
               class="ml-2"
               @click="deleteAccount(index)"
               :title="$t('addBill.deleteAccountTitle')"
             ></v-btn>
           </div>
        </v-col>
      </v-row>
    </div>

    <!-- Actions -->
    <v-row class="mt-4">
      <v-spacer></v-spacer>
      <v-col cols="auto">
        <v-btn
          color="primary"
          @click="addTransaction"
          prepend-icon="mdi-check"
          elevation="2"
        >
          {{ $t("addBill.addTransaction") }}
        </v-btn>
      </v-col>
    </v-row>
  </v-container>
</template>

<script lang="ts" setup>
import { Ref, ref, computed } from "vue";
// import VueDatePicker from "@vuepic/vue-datepicker"; // Removed
// import "@vuepic/vue-datepicker/dist/main.css"; // Removed
import AddBillAccount from "./AddBillAccount.vue";
import { AccountItem } from "./types";
import { invoke } from "@tauri-apps/api/core";
import { useI18n } from 'vue-i18n';
import { format } from 'date-fns';

const { t } = useI18n();

const accountList: Ref<AccountItem[]> = ref([
  {
    name: t("addBillAccount.selectAccount"),
    icon: "/svg/wallet.svg",
  },
]);
const amounts = ref([]);
const date = ref(new Date());
const dateMenu = ref(false);

const formattedDate = computed(() => {
  return date.value ? format(date.value, 'yyyy-MM-dd') : '';
});

const extra = ref("");
const currency = ref("");

const changeAccount = (account: AccountItem, index: string) => {
  const idx = parseInt(index);
  accountList.value[idx] = account;

  // 如果是最后一个账户被选择了，自动添加新的空账户
  if (idx === accountList.value.length - 1) {
    accountList.value.push({
      name: t("addBillAccount.selectAccount"),
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
    .filter(
      (item) =>
        item.account !== t("addBillAccount.selectAccount") && !isNaN(item.amount) && item.amount !== 0,
    );

  if (validAccounts.length === 0) {
    alert(t("addBill.atLeastOneAccount")); // Could replace with a snackbar later
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
      accountList.value = [
        {
          name: t("addBillAccount.selectAccount"),
          icon: "/svg/wallet.svg",
        },
      ];
      amounts.value = [];
      extra.value = "";
      date.value = new Date();
    })
    .catch((error) => {
      console.error("添加交易失败:", error);
      alert(t("addBill.addTransactionFailed"));
    });
};
</script>

<style scoped>
/* VueDatePicker overrides to match Vuetify roughly */
:deep(.dp__input) {
  padding: 10px 12px;
  border-radius: 4px;
  border-color: #9e9e9e; /* Grey darken-1 approx */
  font-family: inherit;
}
:deep(.dp__input:hover) {
    border-color: #212121; /* text-color approx */
}
:deep(.dp__input:focus) {
    border-color: #6200ee; /* Primary */
}
</style>
