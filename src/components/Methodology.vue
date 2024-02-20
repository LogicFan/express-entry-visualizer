<script setup lang="ts">
import { ref } from "vue";
import { NCard } from "naive-ui";

const f1 = ref("$$\\frac{\\text{draw size}}{\\text{pool size with CRS} \\geq \\text{draw CRS}}$$");
const f2 = ref("$$\\frac{\\text{yearly planned size} - \\text{already invited size}}{\\text{number of remaining months}}$$");
</script>

<template>
    <n-card title="Introduction">
        Since IRCC does not much information about Express Entry, lots of data in this website are estimated or derived from official source. Here are the documentation about how each estimated value
        are generated. If you find some better estimation method, please let me know by opening a Issue on GitHub repo.
    </n-card>
    <n-card title="Invitations Tab">
        <h3>Invitation CRS Score</h3>
        Data is from IRCC directly, with the following modification:
        <ul>
            <li>Canada Experience Class (CEC) are renamed as Inland since most of CECs are inland applicants.</li>
            <li>Federal Skill Worker (FSW) are renamed as Oversee since most of FSWs are oversea applicants.</li>
            <li>Federal Skill Trade (FST) and Trade 2023-1 are merged as same category due to their similarity in items of NOC requirement.</li>
            <li>General and No Program Specified are merged because they are the same.</li>
            <li>French 2023-1 and French 2024-1 are merged because they are the same.</li>
        </ul>

        <h3>Invitation Size</h3>
        Same as Invitation CRS Score
    </n-card>
    <n-card title="Candidates Tab">
        <h3>Candidates in the Pool</h3>
        Data is from IRCC directly, with missing value being skipped.
    </n-card>
    <n-card title="Categories Tab">
        <h3>Invitation Size By Categories</h3>
        Data is from IRCC directly, only count draws happens after 252nd rounds (inclusive).

        <h3>Invitation Count By Categories</h3>
        Same as Invitation Size By Categories.

        <h3>Estimated Candidates By Categories</h3>
        This is computed as <vue-mathjax :formula="f1" />

        TODO: This is far from accurate because it does not account for recovery of a particular occupation overtime.

        <h3>Most Recent Draw By Categories</h3>
        Data is from IRCC directly.
    </n-card>
    <n-card title="IRCC Plan Tab">
        <h3>IRCC Departmental Plan</h3>
        For per year data, the data is from IRCC yearly departmental plan directly.
        <br />
        For per month data, it is computed as <vue-mathjax :formula="f2" />
    </n-card>
</template>

<style scoped>
.n-card {
    max-width: 600px;
}
</style>
