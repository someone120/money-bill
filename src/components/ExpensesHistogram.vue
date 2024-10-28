<template>
  <div class="box">
    <v-chart class="chart" :option="option" />
  </div>
</template>
<style lang="css" scoped>
.box {
  height: 200px;
  background-color: white;
  border-radius: 10px;
}
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

use([
  GridComponent,
  CanvasRenderer,
  BarChart,
  TitleComponent,
  TooltipComponent,
  LegendComponent,
]);

provide(THEME_KEY, "auto");

const option: Ref<ECBasicOption> = ref({
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
      data: ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"],
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
      name: "Direct",
      type: "bar",
      barWidth: "20%",
      data: [10, 52, 200, 334, 390, 330, 220],
      animationDelay: function (idx) {
        return idx * 10;
      },
      color: "#60bf23",
    },
    {
      name: "income",
      type: "bar",
      barWidth: "20%",
      data: [10, 52, 200, 334, 390, 330, 220],
      animationDelay: function (idx) {
        return idx * 10;
      },
      color: "#f33c75",
    },
  ],
});
</script>
