<script setup lang="ts">
import { Ref, ref } from "vue";
import {
    NCard,
    NGrid,
    NGridItem,
    NDataTable,
    NSwitch,
    NButton,
    NDropdown,
    NDivider,
} from "naive-ui";
import { Doughnut, Line } from "vue-chartjs";
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
    ChartData,
    ChartOptions,
} from "chart.js";
import "chartjs-adapter-date-fns";
import {
    Draw,
    DrawCategory,
    useCategoryColor,
} from "../composables/Constant.ts";
import { useData } from "../composables/DataLoader.ts";
import { TableColumns } from "naive-ui/es/data-table/src/interface";
import wasm_init, {
    wasm_pool_data,
    wasm_invite_data,
    wasm_category_invite_data,
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

/*** ====== Chart Data Definition ====== ***/
let inviteChartData = wasm_category_invite_data(poolData, inviteData, 0, true);

/*** ====== Callbacks Definition ====== ***/

/*** ====== Chart Config Definition ====== ***/
let percentageScaleConfig = {
    min: 0,
    max: 100,
    ticks: {
        callback: function (value) {
            return value + "%";
        },
    },
};

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
        y: percentageScaleConfig,
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

const categories = [
    DrawCategory.STEM,
    DrawCategory.HEALTH,
    DrawCategory.FRENCH,
    DrawCategory.AGRICULTURE,
    DrawCategory.TRADE,
    DrawCategory.TRANSPORT,
];

const firstCategoryDraw = 252;
const data = (await useData()).filter((e) => e.id >= firstCategoryDraw);

function calcInvitationCountChartProps() {
    const labels = categories.concat([DrawCategory.GENERAL]);

    const dataC = categories.map((c) => data.filter((e) => e.name == c).length);
    const dataG = data.filter((e) => !categories.includes(e.name)).length;

    const datasets = [
        {
            data: dataC.concat([dataG]),
            backgroundColor: labels.map((e) => useCategoryColor(e)),
            borderColor: labels.map((e) => useCategoryColor(e)),
        },
    ];

    return {
        data: {
            labels: labels,
            datasets: datasets,
        } as ChartData<"doughnut", number[], string>,
        options: {
            responsive: false,
            maintainAspectRatio: false,
            plugins: { legend: { position: "right" } },
        } as ChartOptions<"doughnut">,
    };
}

function lastDrawOfCategory(d: Array<Draw>, c: DrawCategory): Draw {
    return d
        .filter((e) => e.name == c)
        .sort((a, b) => b.date.getTime() - a.date.getTime())[0];
}

function calcCandidateSizeChartProps() {
    const labels = categories.concat([DrawCategory.GENERAL]);

    function estimatePercentage(d: Draw): number {
        const selected = d.size;
        const above = d.pool
            .filter((e) => e.min > d.crs)
            .reduce((a, b) => a + b.count, 0);
        const m = d.pool.filter((e) => e.min <= d.crs && d.crs <= e.max)[0];
        const middle = ((m.max - d.crs) / (m.max - m.min)) * m.count;
        return selected / (middle + above);
    }

    const dataC = categories
        .map((c) => lastDrawOfCategory(data, c))
        .map((e) => estimatePercentage(e));
    const dataG = 1 - dataC.reduce((a, b) => a + b, 0);

    const datasets = [
        {
            data: dataC.concat([dataG]),
            backgroundColor: labels.map((e) => useCategoryColor(e)),
            borderColor: labels.map((e) => useCategoryColor(e)),
        },
    ];

    return {
        data: {
            labels: labels,
            datasets: datasets,
        } as ChartData<"doughnut", number[], string>,
        options: {
            responsive: false,
            maintainAspectRatio: false,
            plugins: {
                legend: { position: "right" },
                tooltip: {
                    callbacks: {
                        label: function (context: TooltipItem<"doughnut">) {
                            const percentage = (context.raw as number) * 100;
                            return percentage.toFixed(2) + "%";
                        },
                    },
                },
            },
        } as ChartOptions<"doughnut">,
    };
}

function calcInvitationRecencyTableProps() {
    const datasets = categories
        .map((c) => lastDrawOfCategory(data, c))
        .map(function (e) {
            return {
                category: e.name,
                date: e.date.toISOString().substring(0, 10),
            };
        });

    return {
        columns: [
            { title: "Category", key: "category" },
            {
                title: "Date",
                key: "date",
                defaultSortOrder: "ascend",
                sorter: "default",
            },
        ] as TableColumns,
        data: datasets,
    };
}

let invitationCountChartProps = calcInvitationCountChartProps();
let candidateSizeChartProps = calcCandidateSizeChartProps();
let invitationRecencyTableProps = calcInvitationRecencyTableProps();

let vh = visualViewport.height;
</script>

<template>
    <n-grid :x-gap="12" :y-gap="8" :cols="2">
        <n-grid-item>
            <n-card title="Invitation Size By Categories">
                <template #header-extra>
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
                    <n-divider vertical />
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
        </n-grid-item>
        <n-grid-item>
            <n-card title="Invitation Count By Categories">
                <Doughnut
                    ref="invitationCountChart"
                    :options="invitationCountChartProps.options"
                    :data="invitationCountChartProps.data"
                    :style="{
                        height: '30vh',
                        width: '100%',
                    }"
                />
            </n-card>
        </n-grid-item>
        <n-grid-item>
            <n-card title="Estimated Candidates By Categories">
                <Doughnut
                    id="candidateSizeChart"
                    :options="candidateSizeChartProps.options"
                    :data="candidateSizeChartProps.data"
                    :style="{
                        height: '30vh',
                        width: '100%',
                    }"
                />
            </n-card>
        </n-grid-item>
        <n-grid-item>
            <n-card title="Most Recent Draw By Categories">
                <n-data-table
                    ref="invitationRecencyTable"
                    :columns="invitationRecencyTableProps.columns"
                    :data="invitationRecencyTableProps.data"
                    :bordered="false"
                    :max-height="vh * 0.25"
                    :style="{
                        height: '30vh',
                        width: '100%',
                    }"
                />
            </n-card>
        </n-grid-item>
    </n-grid>
</template>

<style scoped>
.chartAreaWrapper {
    width: 600px;
    overflow-x: scroll;
}
</style>
