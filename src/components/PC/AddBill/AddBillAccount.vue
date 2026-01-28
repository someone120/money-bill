<template>
  <v-menu v-model="isOpen" :close-on-content-click="false" location="bottom start">
    <template v-slot:activator="{ props }">
      <v-text-field
        v-bind="props"
        :model-value="displayAccount.name.split('::').slice(-1)[0] || t('addBillAccount.selectAccount')"
        readonly
        variant="outlined"
        append-inner-icon="mdi-chevron-down"
        hide-details
        density="compact"
        class="cursor-pointer"
        @click="isOpen = true"
      >
        <template v-slot:prepend-inner>
          <v-avatar size="24" color="grey-lighten-3" class="text-caption text-grey-darken-2 mr-2">
             {{ displayAccount.name?.split("::").slice(-1)[0]?.substring(0, 1) || "?" }}
          </v-avatar>
        </template>
      </v-text-field>
    </template>

    <v-card width="350">
      <v-tabs v-model="currentType" density="compact" grow color="primary">
        <v-tab value="income">{{ t('addBillAccount.income') }}</v-tab>
        <v-tab value="expenses">{{ t('addBillAccount.expenses') }}</v-tab>
        <v-tab value="assets">{{ t('addBillAccount.assets') }}</v-tab>
        <v-tab value="liabilities">{{ t('addBillAccount.liabilities') }}</v-tab>
      </v-tabs>

      <v-divider></v-divider>

      <v-list density="compact" class="overflow-y-auto" style="max-height: 300px">
        <v-list-item
          v-if="items.length === 0"
          :title="t('addBillAccount.noAccounts')"
          class="text-center text-grey"
        ></v-list-item>
        
        <v-list-item
          v-for="item in items"
          :key="item.name"
          @click="selectAccount(item)"
          block
        >
           <template v-slot:prepend>
              <v-avatar size="24" color="grey-lighten-3" class="text-caption text-grey-darken-2">
                 {{ item.name.split("::").slice(-1)[0]?.substring(0, 1) || "?" }}
              </v-avatar>
           </template>
           <v-list-item-title>{{ item.name.split("::").slice(-1)[0] }}</v-list-item-title>
        </v-list-item>
      </v-list>
    </v-card>
  </v-menu>
</template>

<script setup lang="ts">
import { ref, watchEffect } from "vue";
import { useI18n } from 'vue-i18n';
import type { AccountItem } from "./types";
import { invoke } from "@tauri-apps/api/core";

const { t } = useI18n();

const emit = defineEmits(["changeAccount"]);
const props = defineProps<{
  id: string;
  displayAccount: AccountItem;
}>();

const isOpen = ref(false);
const currentType = ref("income");
const items = ref<AccountItem[]>([]);

const selectAccount = (account: AccountItem) => {
  emit("changeAccount", account, props.id);
  isOpen.value = false;
};

// 获取账户列表
watchEffect(() => {
  if (!isOpen.value) return;

  items.value = [];
  let method = "get_income_accounts";
  
  switch (currentType.value) {
    case "income":
      method = "get_income_accounts";
      break;
    case "expenses":
      method = "get_expenses_accounts";
      break;
    case "assets":
      method = "get_assets_accounts";
      break;
    case "liabilities":
      method = "get_liabilities_accounts";
      break;
  }

  invoke(method)
    .then((res) => {
      const accounts = res as [{ name: string; icon: string }];
      items.value = accounts.map((it) => ({
        icon: it.icon,
        name: it.name,
      }));
    })
    .catch(() => {
      items.value = [];
    });
});
</script>
