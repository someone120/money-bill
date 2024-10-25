<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import IncomeCost from "./components/IncomeCost.vue";
</script>

<template>
  <div class="container">
    <!-- 边栏 -->
    <div class="side-bar"></div>
    <div class="flex flex1 flex-column">
      <!-- 顶栏 -->
      <div class="flex app-bar"></div>
      <div class="flex flex1">
        <!-- 主要内容 -->
        <div style="flex: 3" class="flex flex-column primary-content mg8">
          <!-- 收入/支出/结余 -->
          <IncomeCost class="desk-only"/>
        </div>
        <!-- 每日支出/收入 -->
        <div style="flex: 2" class="flex flex-column calendar"></div>
      </div>
    </div>
    <!-- <Greet /> -->
  </div>
</template>

<style scoped>
.mgr8 {
  margin-right: 8px;
}

.mg8 {
  margin: 8px;
}

.border {
  border-radius: 10px;
}

.flex-column {
  flex-direction: column;
}

.flex1 {
  flex: 1;
}

.app-bar {
  height: 60px;
  background-color: white;
}

.side-bar {
  width: 200px;
  background-color: white;
}
.flex {
  display: flex;
}
.background {
  background-color: white;
}

.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  height: 100vh;
  width: 100vw;
  margin: 0;
  /* padding-top: 10vh; */
  display: flex;
  /* flex-direction: column; */
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}
@media screen and (max-width: 700px) {
  .desk-only {
    display: none !important;
  }
}
@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}
</style>
