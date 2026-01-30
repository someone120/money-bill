<script setup lang="ts">
import { ref } from "vue";
import { RouterView } from "vue-router";
import SideBar from "./components/side-bar.vue";
import AddBill from "./components/PC/AddBill/AddBill.vue";
import { useDisplay } from "vuetify";

const showAddBill = ref(false);
const drawerOpen = ref(false);
const { mobile } = useDisplay();
</script>

<template>
  <v-app>
    <SideBar v-model="drawerOpen" />
    
    <v-app-bar density="compact" flat border>
       <v-app-bar-nav-icon v-if="mobile" @click="drawerOpen = !drawerOpen"></v-app-bar-nav-icon>
       <v-app-bar-title>{{ $t("app.name") }}</v-app-bar-title>
    </v-app-bar>

    <v-main class="bg-background min-h-screen">
      <div class="flex justify-end p-4">
        <v-btn
          icon="mdi-plus"
          class="!bg-primary !text-primary-foreground"
          elevation="4"
          @click="showAddBill = true"
        ></v-btn>
      </div>
      
      <div class="px-4 pb-4 h-full">
         <RouterView />
      </div>
    </v-main>

    <v-dialog v-model="showAddBill" :fullscreen="mobile" max-width="800" scrollable>
      <v-card class="rounded-lg">
        <v-card-title class="d-flex justify-space-between align-center pa-4">
           <span class="text-h6">{{ $t("addTransaction.title") }}</span>
           <v-btn icon="mdi-close" variant="text" @click="showAddBill = false"></v-btn>
        </v-card-title>
        <v-divider></v-divider>
        <v-card-text class="pa-0">
          <AddBill /> <!-- Consider passing a prop or listener to close dialog on success -->
        </v-card-text>
      </v-card>
    </v-dialog>
  </v-app>
</template>

<style scoped>
/* Custom styles if needed, mostly handled by Vuetify now */
</style>

