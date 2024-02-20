<script setup lang="ts">
import { NCard } from "naive-ui";
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
} from "chart.js";
import "chartjs-adapter-date-fns";

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

import { Draw, Candidates } from "../composables/Constant.ts";
import { useData } from "../composables/DataLoader.ts";

const data = (await useData()).filter((e) => e.pool.length != 0);

function poolArea() {
    const labels = data.map((e) => e.date.getTime());
    const d0 = data[0];
    const range = [...Array(d0.pool.length).keys()];

    function poolAt(d: Draw, i: number): Candidates {
        return d.pool.sort((a, b) => a.min - b.min)[i];
    }

    const dLabel = range
        .map((i) => poolAt(d0, i))
        .map((e) => e.min + "-" + e.max);

    function colorRank(i: number) {
        const p = i / 15;

        var r = 0;
        var g = 0;

        if (p < 0.8) {
            r = 255;
            g = Math.floor((p / 0.8) * 255);
        } else {
            r = 255 - Math.floor(((p - 0.8) / 0.2) * 255);
            g = 255;
        }
        return (
            "#" +
            r.toString(16).padStart(2, "0") +
            g.toString(16).padStart(2, "0") +
            "00"
        );
    }

    const dColor = range.map((i) => colorRank(i));

    const datasets = range.map(function (i) {
        return {
            label: dLabel[i],
            data: data.map((e) => poolAt(e, i).count),
            backgroundColor: dColor[i],
            borderColor: dColor[i],
            fill: true,
            pointStyle: false,
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
                    stacked: true,
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
                        y: { min: 0 },
                    },
                    pan: { enabled: true, mode: "xy" },
                },
            },
        },
    };
}

let poolChart = poolArea();
</script>

<template>
    <n-card title="Candidates in the Pool">
        <Line
            id="poolChart"
            :options="poolChart.options"
            :data="poolChart.data"
            :style="{
                height: '70vh',
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
