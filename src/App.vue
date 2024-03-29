<script setup async lang="ts">
import { NTabs, NTabPane, NConfigProvider, darkTheme } from "naive-ui";

import Invitations from "./components/Invitations.vue";
import Candidates from "./components/Candidates.vue";
import Categories from "./components/Categories.vue";
import Plan from "./components/Plan.vue";
import Version from "./Version.vue";
import Methodology from "./components/Methodology.vue";
</script>

<template>
    <n-config-provider :theme="darkTheme">
        <n-tabs type="line" default-value="invitations" animated>
            <n-tab-pane name="invitations" tab="Invitations">
                <Suspense>
                    <Invitations />
                </Suspense>
            </n-tab-pane>
            <n-tab-pane name="candidates" tab="Candidates">
                <Suspense>
                    <Candidates />
                </Suspense>
            </n-tab-pane>
            <n-tab-pane name="categories" tab="Categories">
                <Suspense>
                    <Categories />
                </Suspense>
            </n-tab-pane>
            <n-tab-pane name="plan" tab="IRCC Plan">
                <Suspense>
                    <Plan />
                </Suspense>
            </n-tab-pane>
            <n-tab-pane name="methodology" tab="Methodology">
                <Suspense>
                    <Methodology />
                </Suspense>
            </n-tab-pane>
        </n-tabs>
        <Version />
    </n-config-provider>
</template>
