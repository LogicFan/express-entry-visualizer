<script setup async lang="ts">
import { NTabs, NTabPane, NConfigProvider, darkTheme } from "naive-ui";

import HelloWorld from "./components/HelloWorld.vue";
import Overview from "./components/Overview.vue";
</script>

<template>
    <n-config-provider :theme="darkTheme">
        <n-tabs type="line" animated>
            <n-tab-pane name="overview" tab="Overview">
                <Suspense>
                    <Overview />
                </Suspense>
            </n-tab-pane>
            <n-tab-pane name="the beatles" tab="the Beatles">
                Hey Jude
            </n-tab-pane>
            <n-tab-pane name="jay chou" tab="Jay Chou"> Qilixiang </n-tab-pane>
        </n-tabs>
        <HelloWorld msg="Vite + Vue" />
    </n-config-provider>
</template>

<style scoped>
.logo {
    height: 6em;
    padding: 1.5em;
    will-change: filter;
    transition: filter 300ms;
}
.logo:hover {
    filter: drop-shadow(0 0 2em #646cffaa);
}
.logo.vue:hover {
    filter: drop-shadow(0 0 2em #42b883aa);
}
</style>
