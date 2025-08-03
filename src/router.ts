import { createMemoryHistory, createRouter } from "vue-router";
import Home from "./components/PC/Home/Home.vue";
// import AccountsList from "./components/AddBill/AccountsList.vue";
import AddBill from "./components/PC/AddBill/AddBill.vue";
import AddBillAccount from "./components/PC/AddBill/AddBillAccount.vue";
import AssetsView from "./components/PC/Assets/AssetsView.vue";
import BudgetView from "./components/PC/Budget/BudgetView.vue";

const routes = [
  { path: "/", component: Home },
  // { path: "/AddBill", component:AccountsList },
  { path: "/assess", component:AddBill },
  { path: "/assets", component: AssetsView },
  { path: "/budget", component: BudgetView },
];

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

export default router;
