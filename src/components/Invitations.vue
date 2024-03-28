<script setup lang="ts">
import { ref, Ref } from "vue";
import { NCard } from "naive-ui";
import { Bar, Line } from "vue-chartjs";
import zoomPlugin from "chartjs-plugin-zoom";
import { FocusScale } from "../composables/FocusScale";
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
    ScaleType,
    ChartOptions,
    TooltipItem,
    ChartTypeRegistry,
} from "chart.js";
import "chartjs-adapter-date-fns";
import { Scale } from "chart.js/auto";
import wasm_init, {
    wasm_invite_data,
    wasm_invitation_x_min,
    wasm_invitation_x_max,
    wasm_invitation_size_data,
    wasm_invitation_score_data,
} from "analyzer";

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
    FocusScale,
    zoomPlugin
);
await wasm_init();

/*** ====== Misc ====== */
let scoreChartRef: Ref<typeof Line> = ref();
let sizeChartRef: Ref<typeof Bar> = ref();
let onFocusChartRef: Ref<typeof Line | typeof Bar> = ref();

const onFocusChart = function (): ChartJS {
    return onFocusChartRef.value.chart;
};
const offFocusChart = function (): ChartJS {
    if (onFocusChartRef.value == scoreChartRef.value) {
        return sizeChartRef.value.chart;
    } else if (onFocusChartRef.value == sizeChartRef.value) {
        return scoreChartRef.value.chart;
    } else {
        return null;
    }
};

/*** ====== Chart Data Definition ====== ***/
let invitationData = await wasm_invite_data();

/*** ====== Callbacks Definition ====== ***/

/** ensure same size tick to algin the diagram */
const callback_scales_y_ticks = function (
    this: Scale,
    tickValue: number,
    _index: unknown,
    _ticks: unknown
) {
    let label = this.getLabelForValue(tickValue);
    return " ".repeat(8 - label.length) + label;
};
const callback_zoom_onPan = (context: { chart: ChartJS }) => {
    let src: ChartJS = context.chart;

    if (onFocusChart() == src) {
        let trg: ChartJS = offFocusChart();
        trg.scales["x"].options.min = src.scales["x"].options.min;
        trg.scales["x"].options.max = src.scales["x"].options.max;
        trg.update("none");
    }
};
const callback_zoom_onZoom = (context: { chart: ChartJS }) => {
    let src: ChartJS = context.chart;

    if (onFocusChart() == src) {
        let trg: ChartJS = offFocusChart();
        trg.scales["x"].options.min = src.scales["x"].options.min;
        trg.scales["x"].options.max = src.scales["x"].options.max;
        trg.update("none");
    }

    let chart: ChartJS = sizeChartRef.value.chart;
    let range = {
        min: chart.scales["x"].min,
        max: chart.scales["x"].max,
    };

    let r = (range.max - range.min) / 1000 / 3600 / 24;

    if (r > 2500) {
        chart.data = wasm_invitation_size_data(invitationData, "m");
        chart.options.plugins.tooltip.callbacks.title =
            callback_tooltip_title_sizeChart_m;
    } else if (r > 583) {
        chart.data = wasm_invitation_size_data(invitationData, "w");
        chart.options.plugins.tooltip.callbacks.title =
            callback_tooltip_title_sizeChart_w;
    } else {
        chart.data = wasm_invitation_size_data(invitationData, "d");
        chart.options.plugins.tooltip.callbacks.title =
            callback_tooltip_title_sizeChart_d;
    }

    chart.update("none");
};

/*** ====== Chart Config Definition ====== ***/
let config_zoom = {
    zoom: {
        wheel: {
            enabled: true,
        },
        mode: "x",
        onZoom: callback_zoom_onZoom,
    },
    limits: {
        x: {
            min: wasm_invitation_x_min(invitationData),
            max: wasm_invitation_x_max(invitationData),
        },
    },
    pan: {
        enabled: true,
        mode: "x",
        onPan: callback_zoom_onPan,
    },
};

const callback_tooltip_title_scoreChart = function <
    T extends keyof ChartTypeRegistry
>(items: TooltipItem<T>[]) {
    return items.map(
        (x) =>
            wasm_invitation_score_data(invitationData).tooltip.title[
                x.dataIndex
            ]
    );
};
let scoreChartConfig = {
    maintainAspectRatio: false,
    scales: {
        x: {
            type: "time",
            min: wasm_invitation_x_min(invitationData),
            max: wasm_invitation_x_max(invitationData),
        },
        y: {
            type: "focus" as ScaleType,
            level: 1.5,
            min: 0,
            max: 1000,
            grid: {
                display: false,
            },
            ticks: {
                // ensure same size tick to algin the diagram
                callback: callback_scales_y_ticks,
            },
        },
    },
    plugins: {
        zoom: config_zoom,
        tooltip: {
            callbacks: {
                title: callback_tooltip_title_scoreChart,
            },
        },
    },
} as ChartOptions<"line">;

const callback_tooltip_title_sizeChart_m = function <
    T extends keyof ChartTypeRegistry
>(items: TooltipItem<T>[]) {
    return items.map(
        (x) =>
            wasm_invitation_size_data(invitationData, "m").tooltip.title[
                x.dataIndex
            ]
    );
};
const callback_tooltip_title_sizeChart_w = function <
    T extends keyof ChartTypeRegistry
>(items: TooltipItem<T>[]) {
    return items.map(
        (x) =>
            wasm_invitation_size_data(invitationData, "w").tooltip.title[
                x.dataIndex
            ]
    );
};
const callback_tooltip_title_sizeChart_d = function <
    T extends keyof ChartTypeRegistry
>(items: TooltipItem<T>[]) {
    return items.map(
        (x) =>
            wasm_invitation_size_data(invitationData, "d").tooltip.title[
                x.dataIndex
            ]
    );
};
let sizeChartConfig = {
    maintainAspectRatio: false,
    scales: {
        x: {
            type: "time",
            min: await wasm_invitation_x_min(invitationData),
            max: await wasm_invitation_x_max(invitationData),
        },
        y: {
            type: "logarithmic",
            min: 0,
            grid: {
                display: false,
            },
            ticks: {
                callback: callback_scales_y_ticks,
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
                onZoom: callback_zoom_onZoom,
            },
            limits: {
                x: {
                    min: await wasm_invitation_x_min(invitationData),
                    max: await wasm_invitation_x_max(invitationData),
                },
            },
            pan: {
                enabled: true,
                mode: "x",
                onPan: callback_zoom_onPan,
            },
        },
        tooltip: {
            callbacks: {
                title: callback_tooltip_title_sizeChart_m,
            },
        },
    },
} as ChartOptions<"bar">;
</script>

<template>
    <n-card title="Invitation CRS Score">
        <div>
            <Line
                ref="scoreChartRef"
                @mouseover="onFocusChartRef = scoreChartRef"
                @mouseleave="onFocusChartRef = null"
                :options="scoreChartConfig"
                :data="wasm_invitation_score_data(invitationData)"
                :style="{
                    height: '30vh',
                    width: '100%',
                }"
            />
        </div>
    </n-card>
    <n-card title="Invitation Size">
        <div>
            <Bar
                ref="sizeChartRef"
                @mouseover="onFocusChartRef = sizeChartRef"
                @mouseleave="onFocusChartRef = null"
                :options="sizeChartConfig"
                :data="wasm_invitation_size_data(invitationData, 'm')"
                :style="{
                    height: '30vh',
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
