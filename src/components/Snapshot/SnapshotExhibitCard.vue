<template>
<n-thing class="card">
  <template #header>{{snap.title}}</template>

  <template #header-extra>
    <!-- 发送按钮 -->
    <n-space>
      <n-popover  v-if="exhibit_model === SnapExhibitModel.ONELINE"
        trigger="hover" placement="top-end">
        <span>多行模式</span>
        <template #trigger>
          <n-button @click.stop="exhibit_model = SnapExhibitModel.MULTLINE"
            type="warning" text size="small">
            <template #icon>
              <n-icon size="16"><LineHorizontal520Filled /></n-icon>
            </template>
          </n-button>
        </template>
      </n-popover>
      <n-popover v-else trigger="hover" placement="top-end">
        <span>单行模式</span>
        <template #trigger>
          <n-button @click.stop="exhibit_model = SnapExhibitModel.ONELINE"
            type="warning" text size="small">
            <template #icon>
              <n-icon size="16"><LineHorizontal120Filled /></n-icon>
            </template>
          </n-button>
        </template>
      </n-popover>
      <n-popover trigger="hover" placement="top-end">
        <span>发送</span>
        <template #trigger>
          <n-button @click.stop="sendCmd(snapshot_str_arr)" type="warning" text size="small">
            <template #icon>
              <n-icon size="16"><PaperPlane /></n-icon>
            </template>
          </n-button>
        </template>
      </n-popover>
    </n-space>
  </template>
  <div v-for="(line, index) in snapshot_str_arr" :key="index">
    {{line}}
  </div>
</n-thing>
</template>
<script lang="ts">
import { defineComponent, ref, computed } from 'vue';
import { getCurrent } from '@tauri-apps/api/window';
import { PaperPlane } from '@vicons/fa';
import {
  LineHorizontal120Filled,
  LineHorizontal520Filled,
} from '@vicons/fluent';
import Snapshot from '@/lib/Snapshot';
import {
  SnapExhibitModel,
  dealCommandExhibit,
} from '@/lib/Util';

export default defineComponent({
  name: 'SnapshotExhibitCard',
  components: {
    LineHorizontal120Filled,
    LineHorizontal520Filled,
    PaperPlane,
  },
  props: {
    snapshot: {
      type: Snapshot,
      require: true,
      default: null,
    },
  },
  setup(props: any) {
    const current = getCurrent();
    const snap = ref(props.snapshot);
    const exhibit_model = ref(SnapExhibitModel.ONELINE);
    const snapshot_str_arr = computed(
      () => dealCommandExhibit(props.snapshot, exhibit_model.value),
    );

    function sendCmd(data: string) {
      current.emit('ssh-data-from-frontend', `${data}\r`);
    }

    return {
      SnapExhibitModel,
      exhibit_model,
      sendCmd,
      snap,
      snapshot_str_arr,
    };
  },
});
</script>
<style scoped style="scss">
.card {
  margin: 5px;
  padding: 5px;
  border: 1px solid rgba(255, 255, 255, 0.24);
  border-radius: 5px;
}
</style>
