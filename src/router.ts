import { createMemoryHistory, createRouter } from "vue-router";
import Home from "./components/PC/Home/Home.vue";
// import AccountsList from "./components/AddBill/AccountsList.vue";
import AddBill from "./components/PC/AddBill/AddBill.vue";
import AddBillAccount from "./components/PC/AddBill/AddBillAccount.vue";
import AssetsView from "./components/PC/Assets/AssetsView.vue";
import BudgetView from "./components/PC/Budget/BudgetView.vue";
import HistoryView from "./components/PC/History/HistoryView.vue";

const routes = [
  { path: "/", component: Home },
  // { path: "/AddBill", component:AccountsList },
  { path: "/assess", component:AddBill },
  { path: "/assets", component: AssetsView },
  { path: "/budget", component: BudgetView },
  { path: "/history", component: HistoryView },
];

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

export default router;
