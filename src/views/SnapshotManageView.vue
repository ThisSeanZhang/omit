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
            style="top: 34px;width: 395px;">
            <!-- <SnapshotPanel :filter="query_key" /> -->
            <SnapshotExhibitCard
            :exhibit_btn="SnapCardExhibitModel.MANAGER_PANEL"
            v-for="snap in snapshots"
            :key="snap.command_name"
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

export default defineComponent({
  name: 'SnapshotCreateView',
  components: {
    SnapshotExhibitCard,
    SnapshotCreatePanel,
  },
  setup() {
    const storage = useStore();
    const query_key = ref('');
    storage.FETCH_SNAPSHOTS();
    const snapshots = computed(() => storage.snapshots.filter(snap => (query_key.value === '' ? true : snap.title.includes(query_key.value))));
    return {
      SnapCardExhibitModel,
      snapshots,
      query_key,
    };
  },
});
</script>
