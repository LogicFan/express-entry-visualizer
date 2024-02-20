<script setup lang="ts">
import { ref, Ref } from "vue";
import { NCard } from "naive-ui";
import { Line } from "vue-chartjs";
import zoomPlugin from "chartjs-plugin-zoom";
import { FocusScale } from "../composables/FocusScale";
import { Chart as ChartJS, Title, Tooltip, Legend, BarElement, LineElement, PointElement, CategoryScale, LinearScale, LogarithmicScale, TimeScale, ChartData, ChartOptions } from "chart.js";
import "chartjs-adapter-date-fns";

ChartJS.register(Title, Tooltip, Legend, BarElement, LineElement, PointElement, CategoryScale, LinearScale, LogarithmicScale, TimeScale, FocusScale, zoomPlugin);

import { DrawCategory, IrccPlan } from "../composables/Constant.ts";
import { useData } from "../composables/DataLoader.ts";
import { Scale } from "chart.js/auto";

// ignore PNP because it is in a separate plan
const data = (await useData()).filter((e) => e.name != DrawCategory.PNP);

const yearMin = Math.min(...data.map((e) => e.date.getFullYear()));
const yearMax = Math.max(...data.map((e) => e.date.getFullYear())) + 3;

function calcXLimit() {
    let xMin = new Date(yearMin, 0).getTime();
    let xMax = new Date(yearMax, 0).getTime();
    let xRange = xMax - xMin;
    return {
        min: xMin - xRange * 0.02,
        max: xMax + xRange * 0.02,
    };
}

const xLimit = calcXLimit();

const callbackYTicks = function (this: Scale, tickValue: number, _index: unknown, _ticks: unknown) {
    // ensure same size tick to algin the diagram
    let label = this.getLabelForValue(tickValue);
    return " ".repeat(8 - label.length) + label;
};

function calcPerYearData() {
    let res: { label: number; planMin: number; planMax: number; actual: number }[] = [];

    for (let y = yearMin; y < yearMax; y++) {
        let label = new Date(y, 6).getTime();
        let planMin = IrccPlan[y].min;
        let planMax = IrccPlan[y].max;

        var actual = null;
        if (Date.now() >= new Date(y, 0).getTime()) {
            actual = data.filter((e) => e.date.getFullYear() == y).reduce((acc, e) => acc + e.size, 0);
        }

        res.push({ label, planMin, planMax, actual });
    }

    return res;
}

function calcPerMonthData() {
    let res: { label: number; planMin: number; planMax: number; actual: number }[] = [];

    for (let y = yearMin; y < yearMax; y++) {
        let planYearMin = IrccPlan[y].min;
        let planYearMax = IrccPlan[y].max;

        for (let m = 0; m < 12; m++) {
            let label = new Date(y, m, 15).getTime();

            let planMin = planYearMin / (12 - m);
            let planMax = planYearMax / (12 - m);

            var actual = null;
            if (Date.now() >= new Date(y, m).getTime()) {
                actual = data.filter((e) => e.date.getFullYear() == y && e.date.getMonth() == m).reduce((acc, e) => acc + e.size, 0);

                planYearMax -= actual;
                planYearMin -= actual;
            } else {
                planYearMax -= planMax;
                planYearMin -= planMin;
            }

            planYearMax = Math.max(0, planYearMax);
            planYearMin = Math.max(0, planYearMin);

            res.push({ label, planMin, planMax, actual });
        }
    }

    return res;
}

function calcPlanChartProps(isMonth: boolean) {
    const chartData = isMonth ? calcPerMonthData() : calcPerYearData();

    const labels = chartData.map((e) => e.label);

    const planMin = {
        label: "Low Range",
        data: chartData.map((e) => e.planMin),
        cubicInterpolationMode: "monotone",
        backgroundColor: "#F4D03F",
        borderColor: "#F4D03F",
        borderDash: [5, 5],
    };

    const planMax = {
        label: "Low Range",
        data: chartData.map((e) => e.planMax),
        cubicInterpolationMode: "monotone",
        backgroundColor: "#F4D03F",
        borderColor: "#F4D03F",
        borderDash: [5, 5],
    };

    const actual = {
        label: "Invitation",
        data: chartData.map((e) => e.actual),
        cubicInterpolationMode: "monotone",
        backgroundColor: "#58D68D ",
        borderColor: "#58D68D ",
    };

    const datasets = [planMin, planMax, actual];

    console.log(chartData);

    const callbackZoom = (context: { chart: ChartJS }) => {
        let chart = context.chart;
        let range = {
            min: chart.scales["x"].min,
            max: chart.scales["x"].max,
        };

        let r = (range.max - range.min) / 1000 / 3600 / 24;

        if (r > 2500) {
            chart.data = calcPlanChartProps(false).data;
        } else {
            chart.data = calcPlanChartProps(true).data;
        }

        chart.update("none");
    };

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
                    min: xLimit.min,
                    max: xLimit.max,
                },
                y: {
                    type: "linear",
                    grid: {
                        display: false,
                    },
                    ticks: {
                        // ensure same size tick to algin the diagram
                        callback: callbackYTicks,
                    },
                },
            },
            plugins: {
                zoom: {
                    zoom: {
                        wheel: {
                            enabled: true,
                        },
                        mode: "x",
                        onZoom: callbackZoom,
                    },
                    limits: {
                        x: {
                            min: xLimit.min,
                            max: xLimit.max,
                        },
                    },
                    pan: {
                        enabled: true,
                        mode: "x",
                    },
                },
            },
        } as ChartOptions<"line">,
    };
}

let planChartProps = calcPlanChartProps(false);
let planChart: Ref<typeof Line> = ref();
let controllingChart = null;
</script>

<template>
    <n-card title="IRCC Departmental Plan">
        <Line
            ref="planChart"
            @mouseover="controllingChart = planChart"
            @mouseleave="controllingChart = null"
            :options="planChartProps.options"
            :data="planChartProps.data"
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
