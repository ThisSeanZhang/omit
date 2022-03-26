<template>
  <div style="width: 380px;">
    <!-- <n-button @click="sendData('ls\r')" dashed>
            Default
    </n-button> -->
    <SnapshotExhibitCard
      v-for="snap in snapshots"
      :key="snap.command_name"
      :snapshot="snap"/>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import { getCurrent } from '@tauri-apps/api/window';
import { invoke } from '@tauri-apps/api/tauri';
import Snapshot from '@/lib/Snapshot';
import SnapshotExhibitCard from '@/components/Snapshot/SnapshotExhibitCard.vue';

export default defineComponent({
  name: 'SnapshotPanle',
  components: {
    SnapshotExhibitCard,
  },
  setup() {
    const current = getCurrent();
    const snapshots = ref(new Array<Snapshot>());

    function sendData(data: string) {
      current.emit('ssh-data-from-frontend', data);
    }

    invoke<Array<Snapshot>>('read_snapshots')
      .then(msg => {
        snapshots.value = msg;
        console.log(msg);
      }).catch((msg:string) => console.log(msg));
    return {
      sendData,
      snapshots,
    };
  },
});
</script>
