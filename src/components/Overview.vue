<script setup lang="ts">
import { ref } from "vue";

import { useData } from "../composables/DataLoader.ts";

const data = await useData();
console.log(data);
</script>

<template>
    <h1>{{ data }}</h1>
</template>

<style scoped></style>
