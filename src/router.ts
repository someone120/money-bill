import { createMemoryHistory, createRouter } from "vue-router";
import Home from "./Home.vue";
import path from "path";
import AddBill from "./components/AddBill.vue";

const routes = [
  { path: "/", component: Home },
  { path: "/AddBill", component:AddBill },
];

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

export default router;
