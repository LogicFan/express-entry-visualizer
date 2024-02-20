<script setup async lang="ts">
import { NTabs, NTabPane, NConfigProvider, darkTheme } from "naive-ui";

import Overview from "./components/Overview.vue";
import Candidates from "./components/Candidates.vue";
</script>

<template>
    <n-config-provider :theme="darkTheme">
        <n-tabs type="line" animated>
            <n-tab-pane name="overview" tab="Overview">
                <Suspense>
                    <Overview />
                </Suspense>
            </n-tab-pane>
            <n-tab-pane name="candidates" tab="Candidates">
                <Suspense>
                    <Candidates />
                </Suspense>
            </n-tab-pane>
            <n-tab-pane name="jay chou" tab="Jay Chou"> Qilixiang </n-tab-pane>
        </n-tabs>
    </n-config-provider>
</template>
