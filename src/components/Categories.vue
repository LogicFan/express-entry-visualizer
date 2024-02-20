<script setup lang="ts">
import { NCard, NGrid, NGridItem } from "naive-ui";
import { Doughnut } from "vue-chartjs";
import {
    Chart as ChartJS,
    Title,
    Tooltip,
    Legend,
    ArcElement,
    CategoryScale,
    LinearScale,
    LogarithmicScale,
    TimeScale,
    TooltipItem,
    ChartTypeRegistry,
} from "chart.js";
import "chartjs-adapter-date-fns";

ChartJS.register(
    Title,
    Tooltip,
    Legend,
    ArcElement,
    CategoryScale,
    LinearScale,
    LogarithmicScale,
    TimeScale
);

import { Draw, DrawName, useDrawColor } from "../composables/Constant.ts";
import { useData } from "../composables/DataLoader.ts";

const categories = [
    DrawName.STEM_2023,
    DrawName.HEALTH_2023,
    DrawName.FRENCH_2023,
    DrawName.AGRICULTURE_2023,
    DrawName.TRADE_2023,
    DrawName.TRANSPORT_2023,
];

const firstCategoryDraw = 252;
const data = (await useData()).filter((e) => e.id >= firstCategoryDraw);

function drawSizePie() {
    const labels = categories.concat([DrawName.GENERAL]);

    const dataC = categories.map((c) =>
        data
            .filter((e) => e.name == c)
            .map((e) => e.size)
            .reduce((a, b) => a + b, 0)
    );
    const dataG = data
        .filter((e) => !categories.includes(e.name))
        .map((e) => e.size)
        .reduce((a, b) => a + b, 0);

    const datasets = [
        {
            data: dataC.concat([dataG]),
            backgroundColor: labels.map((e) => useDrawColor(e)),
            borderColor: labels.map((e) => useDrawColor(e)),
        },
    ];

    return {
        data: {
            labels: labels,
            datasets: datasets,
        },
        options: {
            responsive: false,
            maintainAspectRatio: false,
            plugins: {
                legend: { position: "right" },
                tooltip: {
                    callbacks: {
                        label: function (
                            context: TooltipItem<keyof ChartTypeRegistry>
                        ) {
                            const total = context.dataset.data.reduce(
                                (a, b) => a + b,
                                0
                            );
                            const percentage = (context.raw / total) * 100;
                            return percentage.toFixed(2) + "%";
                        },
                    },
                },
            },
        },
    };
}

function drawCountPie() {
    const labels = categories.concat([DrawName.GENERAL]);

    const dataC = categories.map((c) => data.filter((e) => e.name == c).length);
    const dataG = data.filter((e) => !categories.includes(e.name)).length;

    const datasets = [
        {
            data: dataC.concat([dataG]),
            backgroundColor: labels.map((e) => useDrawColor(e)),
            borderColor: labels.map((e) => useDrawColor(e)),
        },
    ];

    return {
        data: {
            labels: labels,
            datasets: datasets,
        },
        options: {
            responsive: false,
            maintainAspectRatio: false,
            plugins: { legend: { position: "right" } },
        },
    };
}

function poolSizePie() {
    const labels = categories.concat([DrawName.GENERAL]);

    function lastDrawOfCategory(d: Array<Draw>, c: DrawName): Draw {
        return d
            .filter((e) => e.name == c)
            .sort((a, b) => b.date.getDate() - a.date.getDate())[0];
    }

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
            backgroundColor: labels.map((e) => useDrawColor(e)),
            borderColor: labels.map((e) => useDrawColor(e)),
        },
    ];

    return {
        data: {
            labels: labels,
            datasets: datasets,
        },
        options: {
            responsive: false,
            maintainAspectRatio: false,
            plugins: {
                legend: { position: "right" },
                tooltip: {
                    callbacks: {
                        label: function (
                            context: TooltipItem<keyof ChartTypeRegistry>
                        ) {
                            const percentage = context.raw * 100;
                            return percentage.toFixed(2) + "%";
                        },
                    },
                },
            },
        },
    };
}

let drawSizeChart = drawSizePie();
let drawCountChart = drawCountPie();
let poolSizeChart = poolSizePie();
</script>

<template>
    <n-grid :x-gap="12" :y-gap="8" :cols="2">
        <n-grid-item>
            <n-card title="Invitation Size By Categories">
                <Doughnut
                    id="categorySizeChart"
                    :options="drawSizeChart.options"
                    :data="drawSizeChart.data"
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
                    id="categoryCountChart"
                    :options="drawCountChart.options"
                    :data="drawCountChart.data"
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
                    id="categoryPoolChart"
                    :options="poolSizeChart.options"
                    :data="poolSizeChart.data"
                    :style="{
                        height: '30vh',
                        width: '100%',
                    }"
                />
            </n-card>
        </n-grid-item>
        <n-grid-item>
            <n-card title="Candidates in the Pool">
                <Doughnut
                    id="crsChart"
                    :options="drawSizeChart.options"
                    :data="drawSizeChart.data"
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
