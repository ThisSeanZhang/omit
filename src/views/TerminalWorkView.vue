<template>
  <n-layout has-sider sider-placement="right">
    <n-layout-content content-style="padding-right: 20px;">
      <VTerminal style="height: 100%;"></VTerminal>
    </n-layout-content>
    <n-layout-sider
      show-trigger="bar"
      collapsed-width=0
      width="400">
      <n-space style="height: calc(100% - 4px); width: calc(100% - 20px);  padding: 0px 10px;"
        justify="space-between" vertical align="stretch">
        <n-space >
          <SnapshotPanel/>
        </n-space>
        <n-space>
          <n-button dashed>
            Default
          </n-button>
          <n-button type="primary" dashed>
            Primary
          </n-button>
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
      </n-space>
    </n-layout-sider>
  </n-layout>
</template>

<script>

// import {
//   onMounted,
//   ref,
// } from 'vue';

import VTerminal from './VTerminal.vue';
import SnapshotPanel from '@/components/Snapshot/SnapshotPanel.vue';

export default {
  name: 'TerminalWorkView',
  components: {
    VTerminal,
    SnapshotPanel,
  },
  setup() {
    console.log('aaa');
  },
};
</script>
