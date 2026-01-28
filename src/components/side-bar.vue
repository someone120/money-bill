<template>
  <v-navigation-drawer
    :rail="rail"
    permanent
    @click="rail = false"
  >
    <v-list>
      <v-list-item
        :prepend-avatar="rail ? undefined : ''"
        class="px-2"
      >
        <template v-slot:prepend v-if="rail">
           <v-btn icon="mdi-menu" variant="text" @click.stop="rail = !rail"></v-btn>
        </template>
        <template v-slot:append v-if="!rail">
           <v-btn icon="mdi-chevron-left" variant="text" @click.stop="rail = !rail"></v-btn>
        </template>
        <v-list-item-title v-if="!rail" class="text-h6 font-weight-bold text-center">
            {{ $t("app.name") }}
        </v-list-item-title>
      </v-list-item>
    </v-list>

    <v-divider></v-divider>

    <v-list density="compact" nav>
      <v-list-item prepend-icon="mdi-home" :title="$t('home.name')" to="/"></v-list-item>
      <v-list-item prepend-icon="mdi-playlist-plus" :title="$t('addTransaction.title')" to="/assess"></v-list-item>
      <v-list-item prepend-icon="mdi-wallet" :title="$t('assest.name')" to="/assets"></v-list-item>
      <v-list-item prepend-icon="mdi-chart-pie" :title="$t('budget')" to="/budget"></v-list-item>
      <v-list-item prepend-icon="mdi-history" :title="$t('history')" to="/history"></v-list-item>
    </v-list>
  </v-navigation-drawer>
</template>

<script setup lang="ts">
import { ref } from "vue";

const rail = ref(false);
</script>

