<script setup lang="ts">
import { NCard } from "naive-ui";
import { Bar, Line } from "vue-chartjs";
import zoomPlugin from "chartjs-plugin-zoom";
import { SimLogScale } from "../composables/SimLogScale";
import {
    Chart as ChartJS,
    Title,
    Tooltip,
    Legend,
    BarElement,
    LineElement,
    PointElement,
    CategoryScale,
    LinearScale,
    LogarithmicScale,
    TimeScale,
    ChartData,
    ChartOptions,
    CartesianScaleTypeRegistry,
} from "chart.js";
import "chartjs-adapter-date-fns";

ChartJS.register(
    Title,
    Tooltip,
    Legend,
    BarElement,
    LineElement,
    PointElement,
    CategoryScale,
    LinearScale,
    LogarithmicScale,
    TimeScale,
    SimLogScale,
    zoomPlugin
);

import { useDrawColor } from "../composables/Constant.ts";
import { useData } from "../composables/DataLoader.ts";

const data = await useData();

function crsLine() {
    const labels = data.map((e) => e.date.getTime());
    const categories = [...new Set(data.map((e) => e.name))];
    const datasets = categories.map(function (x) {
        return {
            label: x.toString(),
            data: data.map((e) => (e.name == x ? e.crs : null)),
            backgroundColor: useDrawColor(x),
            borderColor: useDrawColor(x),
            spanGaps: true,
        };
    });

    return {
        data: {
            labels: labels,
            datasets: datasets,
        } as ChartData<"line", number[], number>,
        options: {
            responsive: false,
            maintainAspectRatio: false,
            scales: {
                x: {
                    type: "time",
                },
                y: {
                    type: "simlog" as keyof CartesianScaleTypeRegistry,
                    min: -1200,
                    max: 2400,
                    center: 600,
                    delta: 0.1,
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
                    pan: { enabled: true, mode: "x" },
                },
            },
        } as ChartOptions<"line">,
    };
}

function sizeBar() {
    const labels = data.map((e) => e.id);
    const categories = [...new Set(data.map((e) => e.name))];
    const datasets = categories.map(function (x) {
        return {
            label: x.toString(),
            data: data.map((e) => (e.name == x ? e.size : null)),
            backgroundColor: useDrawColor(x),
            borderColor: useDrawColor(x),
            stack: "0",
        };
    });

    return {
        data: {
            labels: labels,
            datasets: datasets,
        } as ChartData<"bar", number[], number>,
        options: {
            responsive: false,
            maintainAspectRatio: false,
            scales: {
                x: {
                    type: "linear",
                },
                y: {
                    min: 0,
                    type: "linear",
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
                    pan: { enabled: true, mode: "x" },
                },
            },
        } as ChartOptions<"bar">,
    };
}

let crsChart = crsLine();
let sizeChart = sizeBar();
</script>

<template>
    <n-card title="Invitation CRS Score">
        <Line
            id="crsChart"
            :options="crsChart.options"
            :data="crsChart.data"
            :style="{
                height: '30vh',
                width: '100%',
            }"
        />
    </n-card>
    <n-card title="Invitation Size">
        <Bar
            id="sizeChart"
            :options="sizeChart.options"
            :data="sizeChart.data"
            :style="{
                height: '30vh',
                width: '100%',
            }"
        />
    </n-card>
</template>

<style scoped>
.chartAreaWrapper {
    width: 600px;
    overflow-x: scroll;
}
</style>
