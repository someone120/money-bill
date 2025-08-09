<template>
  <div class="flex flex-col h-full">
    <div
      class="flex items-center justify-between p-4 border-b-2 border-gray-200"
    >
      <h1 class="text-xl font-semibold">{{ $t("budget") }}</h1>
      <button
        class="px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600"
      >
        {{ $t("addBudget") }}
      </button>
    </div>
    <div class="flex-1 p-4">
      <!-- Budget list will be added here -->
      <div class="grid gap-4">
        <div class="p-4 bg-white rounded-lg shadow">
          <div class="flex justify-between items-center">
            <div>
              <h3 class="font-medium">{{ $t("monthlyBudget") }}</h3>
              <p class="text-gray-600">{{ $t("budgetDate") }}</p>
            </div>
            <div class="text-right">
              <p class="text-lg font-semibold">¥5,000</p>
              <p class="text-sm text-gray-500">¥3,000 / ¥5,000</p>
            </div>
          </div>
          <div class="mt-2 w-full bg-gray-200 rounded-full h-2">
            <div class="bg-blue-500 h-2 rounded-full" style="width: 60%"></div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts"></script>
