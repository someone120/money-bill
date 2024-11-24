<template>
  <div class="flex flex-column bar">
    <div class="flex head">
      <p style="align-items: baseline; margin: auto">Money Bill</p>
    </div>
    <RouterLink to="/" class="routelink">
      <div class="flex item">
        <img src="/svg/home.svg" />
        <p>
          {{ getString("home") }}
        </p>
      </div>
    </RouterLink>
    <RouterLink to="/assess" class="routelink">
      <div class="flex item">
        <img src="/svg/wallet.svg" />
        <p>{{ getString("assest") }}</p>
      </div>
    </RouterLink>
    <RouterLink to="/AddBill" class="routelink">
      <div class="flex item">
        <img src="/svg/wallet.svg" />
        <p>{{ getString("assest") }}</p>
      </div>
    </RouterLink>
  </div>
</template>

<style scoped>
.flex {
  display: flex;
}
.flex-column {
  flex-direction: column;
}
.item {
  justify-content: center;
  margin: 20px;
  margin-bottom: 0;
  background-color: #eee;
  border-radius: 10px;
  height: 40px;
}
.head {
  height: 60px;
  border-bottom: #eee solid 2px;
  border-right: #eee solid 2px;
}
.bar {
  height: 100%;
  width: 100%;
}
p {
  align-items: baseline;
  margin: auto;
  margin-left: 6px;
}
img {
  margin: 6px;
  margin-left: 10px;
}
.routelink {
  text-decoration: none;
  color: #000;
}
</style>

<script setup lang="ts">
import { RouterLink } from "vue-router";
import { i18n } from "../i18n";

let i18 = i18n.getInstace("zh_CN");
function getString(key: string): string {
  return i18.getString(key);
}
</script>
