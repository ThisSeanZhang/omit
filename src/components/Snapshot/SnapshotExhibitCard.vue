<template>
  <n-card>{{snapshot}}</n-card>
</template>
<script lang="ts">
import { defineComponent } from 'vue';
import Snapshot from '@/lib/Snapshot';

export default defineComponent({
  props: {
    snapshot: {
      type: Snapshot,
      require: true,
      default: null,
    },
  },
  setup() {
    console.log();
  },
});
</script>
