<template>
  <div class="h-72 card w-full">
    <v-chart
      class="h-full w-full"
      :option="option"
      :loading="loading"
      autoresize
    />
  </div>
</template>

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
import { invoke } from "@tauri-apps/api/core";
import { useI18n } from "vue-i18n";
import { computed } from "vue";

use([
  GridComponent,
  CanvasRenderer,
  BarChart,
  TitleComponent,
  TooltipComponent,
  LegendComponent,
]);
const { t } = useI18n();
provide(THEME_KEY, "auto");
let loading = ref<boolean>(false);

// Use computed/ref to allow theme updates? For now static is fine or just inline
let op = ref({
  tooltip: {
    trigger: "axis",
    axisPointer: {
      type: "shadow",
    },
    backgroundColor: 'rgba(255, 255, 255, 0.9)',
    borderColor: '#e4e4e7',
    textStyle: {
      color: '#18181b'
    }
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
        t("mon"),
        t("tue"),
        t("wed"),
        t("thu"),
        t("fri"),
        t("sat"),
        t("sun"),
      ],
      axisTick: {
        alignWithLabel: true,
      },
      axisLine: {
        lineStyle: {
            color: '#71717a' /* zinc-500 */
        }
      }
    },
  ],
  yAxis: [
    {
      type: "value",
      splitLine: {
          lineStyle: {
              color: '#e4e4e7' /* zinc-200 */
          }
      }
    },
  ],
  series: [
    {
      name: t("expenses"),
      type: "bar",
      barWidth: "20%",
      data: [0, 0, 0, 0, 0, 0, 0],
      animationDelay: function (idx: number) {
        return idx * 10;
      },
      color: "#e11d48",
      itemStyle: {
          borderRadius: [4, 4, 0, 0]
      }
    },
    {
      name: t("income"),
      type: "bar",
      barWidth: "20%",
      data: [0, 0, 0, 0, 0, 0, 0],
      animationDelay: function (idx: number) {
        return idx * 10;
      },
      color: "#059669",
      itemStyle: {
          borderRadius: [4, 4, 0, 0]
      }
    },
  ],
});
let option: Ref<ECBasicOption> = ref(op);
loading.value = true;
invoke("get_weekly_income_expenses").then((res: any) => {
  loading.value = false;
  console.log(res["expense"]);

  op.value.series[0].data = (res["expenses"] as number[]).map((v) => v * -1);
  op.value.series[1].data = res["income"];
});
</script>
