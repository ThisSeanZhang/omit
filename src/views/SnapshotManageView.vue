<template>
  <n-layout has-sider sider-placement="right">
    <n-layout-content content-style="padding: 10px 20px 0px 10px;">
      <CommandSearchBar v-on:selectCmd="handleUpdateCmd" />
      <SnapshotCreatePanel
      v-if="select_cmd !== null"
      :command="select_cmd" :edit_snap="edit_snap"/>
      <n-result v-else status="404" title="404 资源不存在" description="生活总归带点荒谬">
        <template #footer>
          <n-button>找点乐子吧</n-button>
        </template>
      </n-result>
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
            style="top: 34px;width: 395px;">
            <!-- <SnapshotPanel :filter="query_key" /> -->
            <SnapshotExhibitCard
            :exhibit_btn="SnapCardExhibitModel.MANAGER_PANEL"
            v-for="snap in snapshots"
            :key="snap.snap_id"
            @revise:snap="reviseSnap"
            :snapshot="snap"/>
          </n-layout-content>
        </n-layout>
      </div>
    </n-layout-sider>
  </n-layout>
</template>
<script lang="ts">
import {
  computed,
  defineComponent,
  ref,
} from 'vue';
import {
  SnapCardExhibitModel,
} from '@/lib/Util';
import { useStore } from '@/store/snapshot';
// import SnapshotPanel from '@/components/Snapshot/SnapshotPanel.vue';
import SnapshotCreatePanel from '@/components/Snapshot/SnapshotCreatePanel.vue';
import SnapshotExhibitCard from '@/components/Snapshot/SnapshotExhibitCard.vue';
import CommandSearchBar from '@/components/Command/CommandSearchBar.vue';
import Snapshot from '@/lib/Snapshot';
import Command from '@/lib/Command';

export default defineComponent({
  name: 'SnapshotCreateView',
  components: {
    CommandSearchBar,
    SnapshotExhibitCard,
    SnapshotCreatePanel,
  },
  setup() {
    const storage = useStore();
    const query_key = ref('');
    storage.FETCH_SNAPSHOTS();
    const snapshots = computed(() => storage.snapshots.filter(snap => (query_key.value === '' ? true : snap.title.includes(query_key.value))));

    const select_cmd = ref(null as unknown as Command);
    const edit_snap = ref(null as unknown as Snapshot);

    function handleUpdateCmd(updateCmd: Command) {
      select_cmd.value = updateCmd;
      edit_snap.value = Snapshot.fromCmd(updateCmd);
    }

    function reviseSnap(snap: Snapshot) {
      edit_snap.value = snap.clone();
      console.log(snap);
    }

    return {
      handleUpdateCmd,
      reviseSnap,
      select_cmd,
      edit_snap,
      SnapCardExhibitModel,
      snapshots,
      query_key,
    };
  },
});
</script>
