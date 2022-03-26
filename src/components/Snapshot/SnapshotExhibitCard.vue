<template>
<n-thing class="card" v-for="(line, index) in snapshot_str_arr" :key="index">
  <template #header>{{snap.title}}</template>

  <template #header-extra>
    <!-- 发送按钮 -->
    <n-popover trigger="hover" placement="left">
      <span>发送</span>
      <template #trigger>
        <n-button @click.stop="sendCmd(line)" text size="small">
          <n-icon size="16">
            <PaperPlane />
          </n-icon>
        </n-button>
      </template>
    </n-popover>
  </template>

  {{line}}
</n-thing>
</template>
<script lang="ts">
import { defineComponent, ref, computed } from 'vue';
import { getCurrent } from '@tauri-apps/api/window';
import { PaperPlane } from '@vicons/fa';
import Snapshot from '@/lib/Snapshot';
import {
  SnapExhibitModel,
  dealCommandExhibit,
} from '@/lib/Util';

export default defineComponent({
  components: {
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
