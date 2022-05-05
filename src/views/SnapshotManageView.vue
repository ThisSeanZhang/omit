<template>
  <n-layout has-sider sider-placement="right">
    <n-layout-content content-style="padding-right: 20px;">
      <SnapshotCreatePanel/>
    </n-layout-content>
    <n-layout-sider
      show-trigger="bar"
      collapsed-width=0
      width="400">
      <div style="position: relative;height: calc(100%);">
        <n-layout position="absolute">
          <n-layout-header style="height: 34px; padding-right: 10px;">
            <n-input type="text"
              v-model:value="query_key"
              placeholder="filter"
              clearable
            />
          </n-layout-header>
          <n-layout-content position="absolute"
            style="top: 34px;">
            <SnapshotPanel :filter="query_key" />
          </n-layout-content>
        </n-layout>
      </div>
    </n-layout-sider>
  </n-layout>
</template>
<script lang="ts">
import {
  defineComponent,
  ref,
} from 'vue';
import SnapshotPanel from '@/components/Snapshot/SnapshotPanel.vue';
import SnapshotCreatePanel from '@/components/Snapshot/SnapshotCreatePanel.vue';

export default defineComponent({
  name: 'SnapshotCreateView',
  components: {
    SnapshotPanel,
    SnapshotCreatePanel,
  },
  setup() {
    const query_key = ref('');
    return {
      query_key,
    };
  },
});
</script>
