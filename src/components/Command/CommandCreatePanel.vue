<template>
<n-layout content-style="padding: 24px;" :native-scrollbar="false">
  <n-h2>平山道</n-h2>
  <n-h2>平山道</n-h2>
  <n-h2>平山道</n-h2>
  <n-h2>平山道</n-h2>
  <n-h2>平山道</n-h2>
  <n-h2>平山道</n-h2>
  <n-h2>平山道</n-h2>
  <n-h2>平山道</n-h2>
  <n-h2>平山道</n-h2>
  <n-h2>平山道</n-h2>
  <n-h2>平山道</n-h2>
  <n-h2>平山道</n-h2>
</n-layout>
</template>
<script lang="ts">
import Command from '@/lib/Command';
import {
  defineComponent,
  ref,
} from 'vue';

export default defineComponent({
  setup() {
    const cmd = ref(new Command());
    return {
      cmd,
    };
  },
});
</script>
