<script setup lang="ts">
import { ref, CSSProperties } from "vue";
import { NCard, NSwitch } from "naive-ui";
import { Line } from "vue-chartjs";
import zoomPlugin from "chartjs-plugin-zoom";
import {
    Chart as ChartJS,
    Title,
    Tooltip,
    Legend,
    LineElement,
    PointElement,
    CategoryScale,
    LinearScale,
    LogarithmicScale,
    TimeScale,
    Filler,
    ChartOptions,
    TooltipItem,
} from "chart.js";
import "chartjs-adapter-date-fns";
import wasm_init, {
    wasm_pool_data,
    wasm_invite_data,
    wasm_pool_n,
    wasm_pool_count_x_min,
    wasm_pool_count_x_max,
    wasm_pool_count_y_max,
    wasm_pool_count_data,
    wasm_pool_rate_x_min,
    wasm_pool_rate_x_max,
    wasm_pool_rate_data,
} from "analyzer";

ChartJS.register(
    Title,
    Tooltip,
    Legend,
    LineElement,
    PointElement,
    CategoryScale,
    LinearScale,
    LogarithmicScale,
    TimeScale,
    zoomPlugin,
    Filler
);

await wasm_init();

/*** ====== Misc ====== */
const checkboxStyle = function ({
    focused,
    checked,
}: {
    focused: boolean;
    checked: boolean;
}) {
    const style: CSSProperties = {};
    if (checked) {
        style.background = "#e67e22";
        if (focused) {
            style.boxShadow = "0 0 0 2px #e67e2240";
        }
    } else {
        style.background = "#3498db";
        if (focused) {
            style.boxShadow = "0 0 0 2px #3498db40";
        }
    }
    return style;
};
let isRateChecked = ref(false);

/*** ====== Chart Data Definition ====== ***/
let poolData = await wasm_pool_data();
let inviteData = await wasm_invite_data();
let poolN = wasm_pool_n();
let countChartData = wasm_pool_count_data(poolData);
let rateChartData = wasm_pool_rate_data(poolData, inviteData);

/*** ====== Chart Config Definition ====== ***/
let countChartConfig = {
    maintainAspectRatio: false,
    scales: {
        x: {
            type: "time",
        },
        y: {
            min: 0,
            reverse: true,
        },
    },
    plugins: {
        zoom: {
            zoom: {
                wheel: {
                    enabled: true,
                },
                mode: "xy",
            },
            limits: {
                y: {
                    min: 0,
                    max: wasm_pool_count_y_max(poolData),
                },
                x: {
                    min: wasm_pool_count_x_min(poolData),
                    max: wasm_pool_count_x_max(poolData),
                },
            },
            pan: { enabled: true, mode: "xy" },
        },
        legend: { position: "right" },
        tooltip: {
            enabled: false,
        },
    },
} as ChartOptions<"line">;

const callback_tooltip_title_rateChart = function (
    items: TooltipItem<"line">[]
) {
    return items.map((_) => "Predicted Increase Rate");
};
const callback_tooltip_label_rateChart = function (item: TooltipItem<"line">) {;
    return rateChartData.tooltip.label[item.datasetIndex][0];
};

let rateChartConfig = {
    maintainAspectRatio: false,
    interaction: {
        mode: "nearest",
        axis: "xy",
        intersect: false,
    },
    scales: {
        x: {
            type: "time",
        },
        y: {
            reverse: true,
        },
    },
    plugins: {
        zoom: {
            zoom: {
                wheel: {
                    enabled: true,
                },
                mode: "x",
            },
            limits: {
                x: {
                    min: wasm_pool_rate_x_min(poolData),
                    max: wasm_pool_rate_x_max(poolData),
                },
            },
            pan: { enabled: true, mode: "x" },
        },
        tooltip: {
            filter: function (item) {
                return item.datasetIndex >= poolN;
            },
            callbacks: {
                title: callback_tooltip_title_rateChart,
                label: callback_tooltip_label_rateChart,
            },
        },
        legend: {
            position: "right",
            labels: {
                filter: function (item) {
                    return item.text != "none";
                },
            },
        },
    },
} as ChartOptions<"line">;
</script>

<template>
    <n-card title="Candidates in the Pool">
        <template #header-extra>
            <n-switch
                :round="false"
                :rail-style="checkboxStyle"
                v-model:value="isRateChecked"
            >
                <template #checked> Increase Rate </template>
                <template #unchecked> Total Count </template>
            </n-switch>
        </template>
        <div v-if="isRateChecked">
            <Line
                ref="rateChartRef"
                :options="rateChartConfig"
                :data="rateChartData"
                :style="{
                    height: '70vh',
                    width: '100%',
                }"
            />
        </div>
        <div v-else>
            <Line
                ref="countChartRef"
                :options="countChartConfig"
                :data="countChartData"
                :style="{
                    height: '70vh',
                    width: '100%',
                }"
            />
        </div>
    </n-card>
</template>

<style scoped>
.chartAreaWrapper {
    width: 600px;
    overflow-x: scroll;
}
</style>
