<script setup lang="ts">
import { NCard } from "naive-ui";
import { Bar, Line } from "vue-chartjs";
import zoomPlugin from "chartjs-plugin-zoom";
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
        },
        options: {
            responsive: false,
            maintainAspectRatio: false,
            scales: {
                x: {
                    type: "time",
                },
                y: {
                    min: 0,
                    max: 1200,
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
        },
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
            stack: 0,
        };
    });

    return {
        data: {
            labels: labels,
            datasets: datasets,
        },
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
        },
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
            id="crsChart"
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
