<script setup lang="ts">
import { NCard, NSwitch } from "naive-ui";
import { Line } from "vue-chartjs";
import zoomPlugin from "chartjs-plugin-zoom";
import { Chart as ChartJS, Title, Tooltip, Legend, LineElement, PointElement, CategoryScale, LinearScale, LogarithmicScale, TimeScale, Filler, ChartOptions } from "chart.js";
import "chartjs-adapter-date-fns";
import wasm_init, { wasm_pool_data, wasm_pool_x_min, wasm_pool_x_max, wasm_pool_count_y_max, wasm_pool_count_data } from "analyzer";

ChartJS.register(Title, Tooltip, Legend, LineElement, PointElement, CategoryScale, LinearScale, LogarithmicScale, TimeScale, zoomPlugin, Filler);

await wasm_init();

/*** ====== Chart Data Definition ====== ***/
let poolData = await wasm_pool_data();
// let invitationData = await wasm_invite_data();

/*** ====== Chart Config Definition ====== ***/
let poolCountConfig = {
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
                y: {
                    min: 0,
                    max: wasm_pool_count_y_max(poolData),
                },
                x: {
                    min: wasm_pool_x_min(poolData),
                    max: wasm_pool_x_max(poolData),
                },
            },
            pan: { enabled: true, mode: "xy" },
        },
        tooltip: {
            enabled: false,
        },
    },
} as ChartOptions<"line">;
</script>

<template>
    <n-card title="Candidates in the Pool">
        <template #header-extra>
            <n-switch>
                <template #checked> Big wheels keep on turnin' </template>
                <template #unchecked> Carry me home to see my kin </template>
            </n-switch>
        </template>
        <div>
            <Line
                ref="candidatesChart"
                :options="poolCountConfig"
                :data="wasm_pool_count_data(poolData)"
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
