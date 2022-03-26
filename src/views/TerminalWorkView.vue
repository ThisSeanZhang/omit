<template>
  <n-layout has-sider sider-placement="right">
    <n-layout-content content-style="padding-right: 20px;">
      <VTerminal v-show="exhibit_terminal" style="height: 100%;"></VTerminal>
      <SnapshotCreatePanel v-show="!exhibit_terminal" />
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
            style="top: 34px;bottom: 44px;">
            <SnapshotPanel :filter="query_key" />
          </n-layout-content>
          <n-layout-footer position="absolute" style="height: 44px; padding: 5px">
            <n-space justify="end">
              <n-popover v-if="exhibit_terminal" placement="top-end" trigger="hover">
                <template #trigger>
                  <n-button type="info" @click.stop="exhibit_terminal = !exhibit_terminal" dashed>
                    <template #icon>
                      <n-icon><CameraAdd24Regular /></n-icon>
                    </template>
                  </n-button>
                </template>
                <span>增加快照</span>
              </n-popover>
              <n-popover v-else placement="top-end" trigger="hover">
                <template #trigger>
                  <n-button type="info" @click.stop="exhibit_terminal = !exhibit_terminal" dashed>
                    <template #icon>
                      <n-icon><WindowConsole20Regular /></n-icon>
                    </template>
                  </n-button>
                </template>
                <span>返回终端</span>
              </n-popover>
              <!-- <n-button type="success" dashed>
                Success
              </n-button>
              <n-button type="warning" dashed>
                Warning
              </n-button>
              <n-button round  type="error" dashed>
                Error
              </n-button> -->
            </n-space>
          </n-layout-footer>
        </n-layout>
      </div>
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
import { CameraAdd24Regular, WindowConsole20Regular } from '@vicons/fluent';
import VTerminal from './VTerminal.vue';
import SnapshotPanel from '@/components/Snapshot/SnapshotPanel.vue';
import SnapshotCreatePanel from '@/components/Snapshot/SnapshotCreatePanel.vue';

export default defineComponent({
  name: 'TerminalWorkView',
  components: {
    CameraAdd24Regular,
    WindowConsole20Regular,
    SnapshotCreatePanel,
    VTerminal,
    SnapshotPanel,
  },
  setup() {
    const query_key = ref('');
    // const exhibit_terminal = ref(true);
    const exhibit_terminal = ref(false);
    return {
      exhibit_terminal,
      query_key,
    };
  },
});
</script>
