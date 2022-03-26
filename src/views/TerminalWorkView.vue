<template>
  <n-layout has-sider sider-placement="right">
    <n-layout-content content-style="padding-right: 20px;">
      <VTerminal style="height: 100%;"></VTerminal>
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
            style="top: 34px; bottom: 44px">
            <SnapshotPanel :filter="query_key" />
          </n-layout-content>
          <n-layout-footer position="absolute" style="height: 44px; padding: 5px">
            <n-button type="info" dashed>
              Info
            </n-button>
            <n-button type="success" dashed>
              Success
            </n-button>
            <n-button type="warning" dashed>
              Warning
            </n-button>
            <n-button round  type="error" dashed>
              Error
            </n-button>
          </n-layout-footer>
        </n-layout>
      </div>
      <!-- <n-space
        style="height: calc(100% - 4px);width: calc(100% - 20px);padding: 0px 10px;"
        justify="space-between" vertical align="stretch">
        <n-space >
          <n-scrollbar  style="max-height: 600px" >
            <SnapshotPanel/>
          </n-scrollbar>
        </n-space>
        <n-space>
          <n-button type="info" dashed>
            Info
          </n-button>
          <n-button type="success" dashed>
            Success
          </n-button>
          <n-button type="warning" dashed>
            Warning
          </n-button>
          <n-button round  type="error" dashed>
            Error
          </n-button>
        </n-space>
      </n-space> -->
    </n-layout-sider>
  </n-layout>
</template>

<script lang="ts">
// import {
//   onMounted,
//   ref,
// } from 'vue';

// import { Retweet } from '@vicons/fa';
import { defineComponent, ref, computed } from 'vue';
import VTerminal from './VTerminal.vue';
import SnapshotPanel from '@/components/Snapshot/SnapshotPanel.vue';

export default defineComponent({
  name: 'TerminalWorkView',
  components: {
    VTerminal,
    SnapshotPanel,
  },
  setup() {
    const query_key = ref('');
    return {
      query_key,
    };
  },
});
</script>
