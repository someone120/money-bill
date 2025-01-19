<template>
  <div class="h-[200px] bg-white rounded-[10px] w-full">
    <v-chart class="h-full w-full" :option="option" :loading="loading" autoresize/>
  </div>
</template>
<style lang="css" scoped>
</style>
<script setup lang="ts">
import { use } from "echarts/core";
import { CanvasRenderer } from "echarts/renderers";
import { BarChart } from "echarts/charts";
import {
  TitleComponent,
  TooltipComponent,
  LegendComponent,
} from "echarts/components";
import VChart, { THEME_KEY } from "vue-echarts";
import { ref, provide, Ref } from "vue";
import { GridComponent } from "echarts/components";
import { ECBasicOption } from "echarts/types/dist/shared";
import { i18n } from "../../i18n";
import { invoke } from "@tauri-apps/api/core";
let i18 = i18n.getInstace("zh_CN");
function getString(key: string): string {
  return i18.getString(key);
}
use([
  GridComponent,
  CanvasRenderer,
  BarChart,
  TitleComponent,
  TooltipComponent,
  LegendComponent,
]);

provide(THEME_KEY, "auto");
let loading = ref<boolean>(false);
let op = ref({
  tooltip: {
    trigger: "axis",
    axisPointer: {
      type: "shadow",
    },
  },
  grid: {
    left: "3%",
    right: "4%",
    bottom: "3%",
    top: "10%",
    containLabel: true,
  },
  xAxis: [
    {
      type: "category",
      data: [
        getString("mon"),
        getString("tue"),
        getString("wed"),
        getString("thu"),
        getString("fri"),
        getString("sat"),
        getString("sun"),
      ],
      axisTick: {
        alignWithLabel: true,
      },
    },
  ],
  yAxis: [
    {
      type: "value",
    },
  ],
  series: [
    {
      name: getString("expenses"),
      type: "bar",
      barWidth: "20%",
      data: [0, 0, 0, 0, 0, 0, 0],
      animationDelay: function (idx) {
        return idx * 10;
      },
      color: "#f33c75",
    },
    {
      name: getString("income"),
      type: "bar",
      barWidth: "20%",
      data: [0, 0, 0, 0, 0, 0, 0],
      animationDelay: function (idx) {
        return idx * 10;
      },
      color: "#60bf23",
    },
  ],
});
let option: Ref<ECBasicOption> = ref(op);
loading.value = true;
invoke("get_weekly_income_expenses").then((res: any) => {
  loading.value = false;
  console.log(res["expense"]);
  
  op.value.series[0].data = (res["expenses"] as number[]).map((v) => v *-1);
  op.value.series[1].data = res["income"];
});


</script>
