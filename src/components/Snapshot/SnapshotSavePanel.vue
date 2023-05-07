<template>
  <n-modal
    :show="value"
    preset="card"
    style="width: 50%;"
    :title="i18n.TRANSLATE('snap.save_title')"
    size="small"
    :bordered="false"
    @update:show="$emit('update:value', false)"
  >
    <template #header-extra>
    </template>
    <!-- {{snap}} -->
    <n-input v-model:value="snapshot.title" clearable type="text" :placeholder="i18n.TRANSLATE('snap.snap_title')" />
    <SnapshotExhibitCard :exhibit_btn="SnapCardExhibitModel.SAVE" :snapshot="snapshot" />
    <template #footer>
      <n-space justify="end">
        <!-- TODO when save the snap need validate snap title -->
        <n-button type="success" @click="saveSnap">{{i18n.TRANSLATE('snap.save')}}</n-button>
      </n-space>
    </template>
  </n-modal>
</template>
<script setup lang="ts">
import {
  ref,
  watch,
} from 'vue';
import { SnapCardExhibitModel } from '@/lib/Util';
import Snapshot from '@/lib/Snapshot';
import SnapshotExhibitCard from '@/components/Snapshot/SnapshotExhibitCard.vue';
import { snapStore } from '@/store/snapshot';
import i18nStore from '@/store/i18n';

const i18n = i18nStore();
interface Props {
  value: Boolean;
  snap: Snapshot;
}
const props = withDefaults(defineProps<Props>(), {
  value: () => false,
  snap: () => new Snapshot({}),
})
const emit = defineEmits<{
  (e: 'reflash:snap'): void
  (e: 'update:value', result: Boolean): void
}>()

const storage = snapStore();
const snapshot = ref(props.snap);
watch(
  () => props.snap,
  newOne => {
    snapshot.value = newOne;
  },
);

function saveSnap() {
  storage.SAVE_SNAPSHOTS(snapshot.value);
  // console.log(JSON.stringify(snapshot.value));
  emit('reflash:snap');
  emit('update:value', false);
}
</script>
