<script lang="ts" setup>
import SnapshotCreatePanel from '@/components/Snapshot/SnapshotCreatePanel.vue';
import CommandCreatePanel from '@/components/Command/CommandCreatePanel.vue';
import CommandAddProcess from '@/components/Command/CommandAddProcess.vue';
import {
  ref,
} from 'vue';
import Command from '@/lib/Command';
import Snapshot from '@/lib/Snapshot';

const edit_cmd = ref(new Command());
const snap = ref(Snapshot.fromCmd(edit_cmd.value));

</script>
<template>
    <n-layout has-sider sider-placement="right" style="height: 100%;">
    <n-layout-content
      content-style="padding: 10px 30px 0px 10px;">
      <SnapshotCreatePanel :command="edit_cmd" :edit_snap="snap" />
    </n-layout-content>
    <n-layout-sider
      content-style="height: 100%"
      show-trigger="bar"
      collapsed-width=0
      width="50%">
      <CommandAddProcess :command="edit_cmd" />
    </n-layout-sider>
  </n-layout>
</template>
