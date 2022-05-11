<template>
  <n-layout>
  {{repos}}
  </n-layout>
</template>
<script lang="ts">
import {
  computed,
  defineComponent,
} from 'vue';
import { useStore as RepoStore } from '@/store/repository';

export default defineComponent({
  name: 'CommandManageView',
  setup() {
    const repos_store = RepoStore();
    const repos = computed(() => repos_store.repos);
    return {
      repos,
    };
  },
});
</script>
