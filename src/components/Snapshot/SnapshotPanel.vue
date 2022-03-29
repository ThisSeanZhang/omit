<template>
  <div style="width: 395px;">
    <!-- <n-button @click="sendData('ls\r')" dashed>
            Default
    </n-button> -->
    <!-- <n-input-group>
      <n-input type="text"
        v-model:value="query_key"
        placeholder="filter"
        clearable
      /> -->
      <!-- <n-button type="primary" tertiary @click="query_key = ''" >
        <n-icon>
          <TimesCircleRegular />
        </n-icon>
      </n-button> -->
    <!-- </n-input-group> -->
    <SnapshotExhibitCard
      v-for="snap in snapshots"
      :key="snap.command_name"
      :snapshot="snap"/>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, computed } from 'vue';
import { getCurrent } from '@tauri-apps/api/window';
import { invoke } from '@tauri-apps/api/tauri';
// import { TimesCircleRegular } from '@vicons/fa';
import Snapshot from '@/lib/Snapshot';
import SnapshotExhibitCard from '@/components/Snapshot/SnapshotExhibitCard.vue';

export default defineComponent({
  name: 'SnapshotPanle',
  components: {
    SnapshotExhibitCard,
  },
  props: {
    filter: {
      type: String,
      require: true,
      default: '',
    },
  },
  setup(props: any) {
    const snapshots_raw = ref(new Array<Snapshot>());
    // const query_key = ref('');

    invoke<string>('read_snapshots')
      .then(msg => {
        snapshots_raw.value = JSON.parse(msg);
        console.log(msg);
      }).catch((msg:string) => console.log(msg));

    const snapshots = computed(() => snapshots_raw.value.filter(snap => (props.filter === '' ? true : snap.title.includes(props.filter))));
    // const test = computed(() => query_key.value);
    return {
      // query_key,
      snapshots,
    };
  },
});
</script>
