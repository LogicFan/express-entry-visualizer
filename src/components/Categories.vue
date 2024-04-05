<script setup lang="ts">
import { Ref, ref } from "vue";
import { NCard, NSwitch, NButton, NDropdown, NDivider } from "naive-ui";
import { Line } from "vue-chartjs";
import {
    Chart as ChartJS,
    Title,
    Tooltip,
    Legend,
    Filler,
    ArcElement,
    CategoryScale,
    LinearScale,
    LogarithmicScale,
    TimeScale,
    TooltipItem,
    ChartOptions,
} from "chart.js";
import "chartjs-adapter-date-fns";
import wasm_init, {
    wasm_pool_data,
    wasm_invite_data,
    wasm_category_invite_data,
    wasm_category_pool_data,
    wasm_category_years,
} from "analyzer";

ChartJS.register(
    Title,
    Tooltip,
    Legend,
    Filler,
    ArcElement,
    CategoryScale,
    LinearScale,
    LogarithmicScale,
    TimeScale
);
await wasm_init();
let poolData = await wasm_pool_data();
let inviteData = await wasm_invite_data();

/*** ====== Misc ====== */
let categoryYears = wasm_category_years(inviteData);

let inviteChartRef: Ref<typeof Line> = ref();
let inviteChartPNP = ref(true);
let inviteChartYear = ref({ label: "all", key: 0 });
function updateInviteChart() {
    inviteChartData = wasm_category_invite_data(
        poolData,
        inviteData,
        inviteChartYear.value.key,
        inviteChartPNP.value
    );
    let chart: ChartJS = inviteChartRef.value.chart;
    chart.data = inviteChartData;
    chart.update("none");
}
let poolChartRef: Ref<typeof Line> = ref();
let poolChartYear = ref({ label: "all", key: 0 });
function updatePoolChart() {
    poolChartData = wasm_category_pool_data(
        poolData,
        inviteData,
        poolChartYear.value.key
    );
    let chart: ChartJS = poolChartRef.value.chart;
    chart.data = poolChartData;
    chart.update("none");
}

/*** ====== Chart Data Definition ====== ***/
let inviteChartData = wasm_category_invite_data(poolData, inviteData, 0, true);
let poolChartData = wasm_category_pool_data(poolData, inviteData, 0);

/*** ====== Callbacks Definition ====== ***/

/*** ====== Chart Config Definition ====== ***/
let callback_tooltip_title_inviteChart = function (
    items: TooltipItem<"line">[]
) {
    return items.map((x) => inviteChartData.tooltip.title[0][x.dataIndex]);
};
let callback_tooltip_label_inviteChart = function (item: TooltipItem<"line">) {
    return inviteChartData.tooltip.label[item.datasetIndex][item.dataIndex];
};
let inviteChartConfig = {
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
            min: 0,
            max: 100,
            ticks: {
                callback: function (value) {
                    return value + "%";
                },
            },
        },
    },
    plugins: {
        legend: { position: "right" },
        tooltip: {
            callbacks: {
                title: callback_tooltip_title_inviteChart,
                label: callback_tooltip_label_inviteChart,
            },
        },
    },
} as ChartOptions<"line">;
let callback_tooltip_title_poolChart = function (items: TooltipItem<"line">[]) {
    return items.map((x) => poolChartData.tooltip.title[0][x.dataIndex]);
};
let callback_tooltip_label_poolChart = function (item: TooltipItem<"line">) {
    return poolChartData.tooltip.label[item.datasetIndex][item.dataIndex];
};
let poolChartConfig = {
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
            min: 0,
            ticks: {
                callback: function (value) {
                    return value + "%";
                },
            },
        },
    },
    plugins: {
        legend: { position: "right" },
        tooltip: {
            callbacks: {
                title: callback_tooltip_title_poolChart,
                label: callback_tooltip_label_poolChart,
            },
        },
    },
} as ChartOptions<"line">;
</script>

<template>
    <div>
        <n-card title="Invitations By Categories">
            <template #header-extra>
                <n-switch
                    :round="false"
                    :value="inviteChartPNP"
                    @update:value="
                        (value) => {
                            inviteChartPNP = value;
                            updateInviteChart();
                        }
                    "
                >
                    <template #checked> PNP </template>
                    <template #unchecked> PNP </template>
                </n-switch>
                <n-divider vertical />
                <n-dropdown
                    :options="categoryYears"
                    @select="
                            (key: number) => { 
                                inviteChartYear = { label: (key == 0 ? 'all' : key.toString()), key: key }; 
                                updateInviteChart();
                            }
                        "
                >
                    <n-button>{{ inviteChartYear.label }}</n-button>
                </n-dropdown>
            </template>
            <Line
                ref="inviteChartRef"
                :options="inviteChartConfig"
                :data="inviteChartData"
                :style="{
                    height: '30vh',
                    width: '100%',
                }"
            />
        </n-card>
    </div>
    <div>
        <n-card title="Candidates By Categories">
            <template #header-extra>
                <n-dropdown
                    :options="categoryYears"
                    @select="
                            (key: number) => { 
                                poolChartYear = { label: (key == 0 ? 'all' : key.toString()), key: key }; 
                                updatePoolChart();
                            }
                        "
                >
                    <n-button>{{ poolChartYear.label }}</n-button>
                </n-dropdown>
            </template>
            <Line
                ref="poolChartRef"
                :options="poolChartConfig"
                :data="poolChartData"
                :style="{
                    height: '30vh',
                    width: '100%',
                }"
            />
        </n-card>
    </div>
</template>

<style scoped>
.chartAreaWrapper {
    width: 600px;
    overflow-x: scroll;
}
</style>
