<template>
  <div class="flex flex-col h-full" :class="isCollapsed ? 'w-16' : 'w-36'">
    <div class="flex h-16 border-b-2 border-gray-200">
      <p v-if="!isCollapsed" class="m-auto">Money Bill</p>
      <button
        @click="toggleCollapse"
        class="m-auto p-1 rounded-full hover:bg-gray-200"
        :class="isCollapsed ? 'ml-4' : 'ml-auto mr-2'"
      >
        <svg
          v-if="isCollapsed"
          xmlns="http://www.w3.org/2000/svg"
          class="h-5 w-5"
          viewBox="0 0 20 20"
          fill="currentColor"
        >
          <path
            fill-rule="evenodd"
            d="M12.707 5.293a1 1 0 010 1.414L9.414 10l3.293 3.293a1 1 0 01-1.414 1.414l-4-4a1 1 0 010-1.414l4-4a1 1 0 011.414 0z"
            clip-rule="evenodd"
          />
        </svg>
        <svg
          v-else
          xmlns="http://www.w3.org/2000/svg"
          class="h-5 w-5"
          viewBox="0 0 20 20"
          fill="currentColor"
        >
          <path
            fill-rule="evenodd"
            d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z"
            clip-rule="evenodd"
          />
        </svg>
      </button>
    </div>
    <RouterLink
      to="/"
      class="no-underline text-black"
      active-class="active-link"
    >
      <div
        class="flex items-center justify-center h-10 mt-5 mb-0 mx-5 rounded-lg"
        :class="{
          'bg-gray-200': $route.path === '/',
          'mx-2': isCollapsed,
        }"
      >
        <img src="/svg/home.svg" class="ml-2.5 mr-1.5" />
        <p v-if="!isCollapsed" class="m-auto ml-1.5">
          {{ $t("home.name") }}
        </p>
      </div>
    </RouterLink>
    <RouterLink
      to="/assess"
      class="no-underline text-black"
      active-class="active-link"
    >
      <div
        class="flex items-center justify-center h-10 mt-5 mb-0 mx-5 rounded-lg"
        :class="{
          'bg-gray-200': $route.path === '/assess',
          'mx-2': isCollapsed,
        }"
      >
        <img src="/svg/add.svg" class="ml-2.5 mr-1.5" />
        <p v-if="!isCollapsed" class="m-auto ml-1.5">
          {{ $t("addTransaction.title") }}
        </p>
      </div>
    </RouterLink>
    <RouterLink
      to="/assets"
      class="no-underline text-black"
      active-class="active-link"
    >
      <div
        class="flex items-center justify-center h-10 mt-5 mb-0 mx-5 rounded-lg"
        :class="{
          'bg-gray-200': $route.path === '/assets',
          'mx-2': isCollapsed,
        }"
      >
        <img src="/svg/wallet.svg" class="ml-2.5 mr-1.5" />
        <p v-if="!isCollapsed" class="m-auto ml-1.5">{{ $t("assest.name") }}</p>
      </div>
    </RouterLink>
    <RouterLink
      to="/budget"
      class="no-underline text-black"
      active-class="active-link"
    >
      <div
        class="flex items-center justify-center h-10 mt-5 mb-0 mx-5 rounded-lg"
        :class="{
          'bg-gray-200': $route.path === '/budget',
          'mx-2': isCollapsed,
        }"
      >
        <img src="/svg/budget.svg" class="ml-2.5 mr-1.5" />
        <p v-if="!isCollapsed" class="m-auto ml-1.5">{{ $t("budget") }}</p>
      </div>
    </RouterLink>
    <RouterLink
      to="/history"
      class="no-underline text-black"
      active-class="active-link"
    >
      <div
        class="flex items-center justify-center h-10 mt-5 mb-0 mx-5 rounded-lg"
        :class="{
          'bg-gray-200': $route.path === '/history',
          'mx-2': isCollapsed,
        }"
      >
        <img src="/svg/history.svg" class="ml-2.5 mr-1.5" />
        <p v-if="!isCollapsed" class="m-auto ml-1.5">{{ $t("history") }}</p>
      </div>
    </RouterLink>
  </div>
</template>

<script setup lang="ts">
import { RouterLink } from "vue-router";
import { useRoute } from "vue-router";
import { ref } from "vue";

const $route = useRoute();
const isCollapsed = ref(false);

const toggleCollapse = () => {
  isCollapsed.value = !isCollapsed.value;
};
</script>
