<script lang="ts" setup>
import { ref } from 'vue';

const current = ref(1);
const currentStatus = ref('process');
enum PROC {
    BASE_INFO = "Base",
    PARAMS_INFO = "",
    OPTIONS_INFO = "",
    PER_VIEW = "",
    DONE = "",
}
const procedure = [
PROC.BASE_INFO,
PROC.PARAMS_INFO,
PROC.OPTIONS_INFO,
PROC.PER_VIEW,
PROC.DONE,
    // Symbol('start'), // BASE_INFO
    // Symbol('add params'), // PARAMS_INFO
    // Symbol('add options'), // OPTIONS_INFO
    // Symbol('per view'), // PER_VIEW
    // Symbol('Done!'), // DONE
]
</script>
<template>
<n-layout content-style="padding: 24px;height: 100%;" :native-scrollbar="false" >
  <n-spce vertical style="height: 100%;">
    <n-steps :current="current" :status="currentStatus" size="small">
        <n-step v-for="step in procedure"
            :title="step"
        ></n-step>
    </n-steps>
    <n-space>
        aaaa
    </n-space>
  </n-spce>
</n-layout>
</template>

