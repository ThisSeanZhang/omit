<template>
    <n-layout has-sider sider-placement="right">
    <n-layout-content
    content-style="padding: 10px 30px 0px 10px;display: flex;flex-direction: column;">
      <SnapshotCreatePanel :command="edit_cmd" :edit_snap="snap" />
    </n-layout-content>
    <n-layout-sider
      show-trigger="bar"
      collapsed-width=0
      width="50%">
      <CommandCreatePanel :command="edit_cmd" />
    </n-layout-sider>
  </n-layout>
</template>
<script lang="ts">
import SnapshotCreatePanel from '@/components/Snapshot/SnapshotCreatePanel.vue';
import CommandCreatePanel from '@/components/Command/CommandCreatePanel.vue';
import {
  defineComponent,
  ref,
} from 'vue';
import Command from '@/lib/Command';
import Snapshot from '@/lib/Snapshot';

export default defineComponent({
  name: 'CommandCreateView',
  components: {
    SnapshotCreatePanel,
    CommandCreatePanel,
  },
  setup() {
    const edit_cmd = ref(new Command());
    const snap = ref(Snapshot.fromCmd(edit_cmd.value));

    return {
      edit_cmd,
      snap,
    };
  },
});
</script>
