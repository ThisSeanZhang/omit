<template>
  <n-modal
    :show="value"
    preset="card"
    style="width: 50%;"
    title="保存快照"
    size="small"
    :bordered="false"
    :segmented="segmented"
    @update:show="$emit('update:value', false)"
  >
    <template #header-extra>
    </template>
    <!-- {{snap}} -->
    <n-input v-model:value="snapshot.title" type="text" placeholder="输入Title" />
    <SnapshotExhibitCard :exhibit_btn="SnapCardExhibitModel.SAVE" :snapshot="snapshot" />
    <template #footer>
      <n-space justify="end">
        <!-- TODO when save the snap need validate snap title -->
        <n-button type="success" @click="saveSnap">保存</n-button>
      </n-space>
    </template>
  </n-modal>
</template>
<script lang="ts">
import Snapshot from '@/lib/Snapshot';
import {
  SnapCardExhibitModel,
} from '@/lib/Util';
import {
  defineComponent,
  ref,
  watch,
} from 'vue';
import { useStore } from '@/store/snapshot';
import SnapshotExhibitCard from '@/components/Snapshot/SnapshotExhibitCard.vue';

export default defineComponent({
  name: 'SnapshotSavePanel',
  components: {
    SnapshotExhibitCard,
  },
  props: {
    value: {
      type: Boolean,
      default: () => false,
    },
    snap: {
      type: Snapshot,
      default: () => new Snapshot(),
    },
  },
  setup(props: any, { emit }) {
    const storage = useStore();
    const snapshot = ref(props.snap);
    watch(
      () => props.snap,
      newOne => {
        snapshot.value = newOne;
      },
    );

    function saveSnap() {
      storage.SAVE_SNAPSHOTS(snapshot.value);
      emit('update:value', false);
    }
    return {
      saveSnap,
      snapshot,
      SnapCardExhibitModel,
    };
  },
});
</script>
