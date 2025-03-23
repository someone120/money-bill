import { createMemoryHistory, createRouter } from "vue-router";
import Home from "./components/Home/Home.vue";
import path from "path";
// import AccountsList from "./components/AddBill/AccountsList.vue";
import AddBill from "./components/AddBill/AddBill.vue";
import AddBillAccount from "./components/AddBill/AddBillAccount.vue";

const routes = [
  { path: "/", component: Home },
  // { path: "/AddBill", component:AccountsList },
  { path: "/assess", component:AddBill },
];

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

export default router;
