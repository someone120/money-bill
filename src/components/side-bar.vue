<template>
  <div class="flex flex-col h-full w-36">
    <div class="flex h-16 border-b-2 border-gray-200">
      <p class="m-auto">Money Bill</p>
    </div>
    <RouterLink to="/" class="no-underline text-black">
      <div
        class="flex items-center justify-center h-10 mt-5 mb-0 mx-5 bg-gray-200 rounded-lg"
      >
        <img src="/svg/home.svg" class="ml-2.5 mr-1.5" />
        <p class="m-auto ml-1.5">
          {{ $t("home.name") }}
        </p>
      </div>
    </RouterLink>
    <RouterLink to="/assess" class="no-underline text-black">
      <div
        class="flex items-center justify-center h-10 mt-5 mb-0 mx-5 bg-gray-200 rounded-lg"
      >
        <img src="/svg/wallet.svg" class="ml-2.5 mr-1.5" />
        <p class="m-auto ml-1.5">{{ $t("assest.name") }}</p>
      </div>
    </RouterLink>
    <RouterLink to="/assets" class="no-underline text-black">
      <div
        class="flex items-center justify-center h-10 mt-5 mb-0 mx-5 bg-gray-200 rounded-lg"
      >
        <img src="/svg/wallet.svg" class="ml-2.5 mr-1.5" />
        <p class="m-auto ml-1.5">{{ $t("assest.name") }}</p>
      </div>
    </RouterLink>
    <RouterLink to="/budget" class="no-underline text-black">
      <div
        class="flex items-center justify-center h-10 mt-5 mb-0 mx-5 bg-gray-200 rounded-lg"
      >
        <img src="/svg/wallet.svg" class="ml-2.5 mr-1.5" />
        <p class="m-auto ml-1.5">{{ $t("assest.name") }}</p>
      </div>
    </RouterLink>
  </div>
</template>

<script setup lang="ts">
import { RouterLink } from "vue-router";
</script>
