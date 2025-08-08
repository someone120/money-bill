<template>
  <div class="flex flex-col w-full h-full p-4">
    <div class="text-xl font-bold mb-4">{{ $t("history") }}</div>
    <div
      v-if="transactions.length === 0"
      class="text-gray-500 text-center py-8"
    >
      {{ $t("no_transactions") }}
    </div>
    <div v-else class="overflow-y-auto">
      <div
        v-for="transaction in transactions"
        :key="transaction.id"
        class="border rounded-lg p-4 mb-3 bg-white shadow-sm"
      >
        <!-- Transaction header with toggle button -->
        <div class="flex justify-between items-center mb-2">
          <div class="font-semibold">{{ transaction.date }}</div>
          <div class="flex items-center">
            <span class="text-red-600 font-bold text-lg mr-2">
              {{ formatAmount(getTotalExpenses(transaction.details)) }}
            </span>
            <div class="text-sm text-gray-500 mr-2">
              {{ transaction.extra }}
            </div>
            <button
              @click="toggleTransaction(transaction.id)"
              class="text-blue-500 hover:text-blue-700"
            >
              {{ collapsedTransactions[transaction.id] ? "▼" : "▲" }}
            </button>
          </div>
        </div>

        <!-- Collapsed view: Show expenses summary -->
        <div v-if="collapsedTransactions[transaction.id]"></div>

        <!-- Expanded view: Show all details in two columns -->
        <div v-else class="grid grid-cols-2 gap-4">
          <!-- Income column (positive balance) -->
          <div class="border-r pr-2">
            <div class="text-green-600 font-medium mb-2">{{ $t("income") }}</div>
            <div
              v-for="detail in transaction.details.filter(d => d.balance > 0)"
              :key="detail.id"
              class="flex items-center mb-2"
            >
              <img
                :src="getAccountIcon(detail.account)"
                class="w-6 h-6 mr-2"
                :alt="detail.account"
              />
              <div class="flex flex-col">
                <div class="text-sm font-medium">{{ detail.account }}</div>
                <div class="text-green-600">
                  {{ formatAmount(detail.balance) }}
                </div>
              </div>
            </div>
          </div>
          
          <!-- Expenses column (negative balance) -->
          <div class="pl-2">
            <div class="text-red-600 font-medium mb-2">{{ $t("expenses") }}</div>
            <div
              v-for="detail in transaction.details.filter(d => d.balance < 0)"
              :key="detail.id"
              class="flex items-center mb-2"
            >
              <img
                :src="getAccountIcon(detail.account)"
                class="w-6 h-6 mr-2"
                :alt="detail.account"
              />
              <div class="flex flex-col">
                <div class="text-sm font-medium">{{ detail.account }}</div>
                <div class="text-red-600">
                  {{ formatAmount(Math.abs(detail.balance)) }}
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, reactive } from "vue";
import { invoke } from "@tauri-apps/api/core";

interface TransactionDetail {
  id: string;
  account: string;
  balance: number;
}

interface Transaction {
  id: string;
  date: string;
  extra: string;
  details: TransactionDetail[];
}

const transactions = ref<Transaction[]>([]);
// Object to track collapsed state of transactions
const collapsedTransactions = reactive<Record<string, boolean>>({});

// Fetch transaction history from the backend
const fetchTransactions = async () => {
  try {
    // This would need to be implemented in the Rust backend
    const result: any[] = await invoke("get_transaction_history");
    transactions.value = result.map((item) => ({
      id: item.id,
      date: item.date,
      extra: item.extra,
      details: item.details.map((detail: any) => ({
        id: detail.id,
        account: detail.account,
        balance: detail.balance,
      })),
    }));

    // Initialize all transactions as collapsed
    transactions.value.forEach((transaction) => {
      collapsedTransactions[transaction.id] = true;
    });
  } catch (error) {
    console.error("Failed to fetch transactions:", error);
  }
};

// Toggle transaction collapse state
const toggleTransaction = (id: string) => {
  collapsedTransactions[id] = !collapsedTransactions[id];
};

// Get account icon based on account name
const getAccountIcon = (accountName: string) => {
  // For now, we'll use a default icon
  // In a real implementation, this would map to specific icons
  return "/svg/wallet.svg";
};

// Format amount with proper sign
const formatAmount = (amount: number) => {
  return amount.toFixed(2);
};

// Calculate total expenses
const getTotalExpenses = (details: TransactionDetail[]) => {
  return details
    .filter((detail) => detail.account.split("::")[0] === "expenses")
    .reduce((sum, detail) => sum - detail.balance, 0);
};

onMounted(() => {
  fetchTransactions();
});
</script>
