<template>
  <div class="rounded w-full">
    <div
      class="flex rounded w-full items-center border border-gray-300 bg-white"
      @click="open"
    >
      <div class="rounded w-9 h-9 m-1 justify-center items-center flex">
        <img
          v-if="displayAccount.icon != ''"
          :src="displayAccount.icon"
          class="w-5 h-5 object-contain"
        />
        <span v-else class="text-gray-500 text-lg">{{
          displayAccount.name?.split("::").at(-1)?.substring(0, 1)
        }}</span>
      </div>
      <div class="flex justify-center items-center">
        <span>
          {{ displayAccount.name.split("::").slice(-1)[0] }}
        </span>
      </div>
      <div class="flex ml-auto w-9 h-9">
        <img src="/svg/more.svg" alt="" />
      </div>
    </div>
    <Transition name="accounts">
      <div
        class="flex flex-col box-border"
        style="position: absolute; z-index: 10"
        v-show="isOpen"
      >
        <AccountTab @changeType="handleTypeChange" />
        <AccountsItems :type="currentType" @select="handleSelect" />
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watchEffect } from "vue";
import type { AccountItem } from "./types";

import AccountTab from "./AccountTab.vue";
import AccountsItems from "./AccountsItems.vue";

const emit = defineEmits(["changeAccount"]);
const props = defineProps(["id", "displayAccount"]);
const displayAccount = ref(props.displayAccount);
watchEffect(() => (displayAccount.value = props.displayAccount));

let isOpen = ref(false);
const currentType = ref("income");
const handleTypeChange = (type: string) => (currentType.value = type);

const handleSelect = (account: AccountItem) => {
  // displayAccount.value=account
  // console.log(account.icon);
  isOpen.value = !isOpen.value;
  emit("changeAccount", account, props.id);
};

function open() {
  isOpen.value = !isOpen.value;
}
</script>

<style>
.accounts-enter-active,
.accounts-leave-active {
  transition: max-height 0.3s, opacity 0.3s;
}

.accounts-enter-from,
.accounts-leave-to {
  opacity: 0;
}
</style>
