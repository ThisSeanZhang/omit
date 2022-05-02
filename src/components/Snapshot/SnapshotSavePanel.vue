<template>
  <n-modal
    :show="value"
    class="custom-card"
    preset="card"
    :style="bodyStyle"
    title="卡片预设"
    size="huge"
    :bordered="false"
    :segmented="segmented"
    @update:show="$emit('update:value', false)"
  >
    <template #header-extra>
      噢!
    </template>
    {{snap}}
    内容
    <template #footer>
      尾部<n-button @click="saveSnap">Default</n-button>
    </template>
  </n-modal>
</template>
<script lang="ts">
import Snapshot from '@/lib/Snapshot';
import {
  defineComponent,
  ref,
  watch,
} from 'vue';
import { useStore } from '@/store/snapshot';

export default defineComponent({
  name: 'SnapshotSavePanel',
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
  setup(props: any) {
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
    }
    return {
      saveSnap,
      snapshot,
    };
  },
});
</script>
