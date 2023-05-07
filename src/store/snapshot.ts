import { invoke } from '@tauri-apps/api/tauri';
import { defineStore } from 'pinia';
import Snapshot from '@/lib/Snapshot';
import { useSettingStore } from './setting'
import { computed, ref } from 'vue';

export const snapStore = defineStore('snap', () => {
  // util
  const settingStore = useSettingStore();

  // const
  const raw_snaps = ref<Snapshot[]>([]);

  // getter
  const snapshots = computed(() => raw_snaps.value);

  // actions
  async function FETCH_SNAPSHOTS(): Promise<void> {
    try {
      // console.log(`${settingStore.userRepoPath}${sep}${SNAPSHOT_FILE}`);
      let snaps_data = await invoke<string[]>('read_snapshots', {
        path: settingStore.setting.snapshots_path,
      });
      // console.log(`snaps_data: ${snaps_data}`);
      raw_snaps.value = snaps_data.map((e: string) => new Snapshot(JSON.parse(e)));
    } catch (e) {
      console.log(e);
    }
  }

  async function SAVE_SNAPSHOTS(snap: Snapshot): Promise<void> {
    try {
      await invoke<string>('append_snapshots', {
        path: settingStore.setting.snapshots_path,
        snapshots: [[snap.snap_id, JSON.stringify(snap)]]
      });
      // await invoke<string>('save_snapshots', { snaps: JSON.stringify(save, null, 2) });
      raw_snaps.value.push(snap);
    } catch (e) {
      console.log(e);
    }
  }

  async function REMOVE_SNAPSHOTS(snap_id: string): Promise<void> {
    try {
      await invoke<string>('delete_snapshot', {
        path: settingStore.setting.snapshots_path,
        id: snap_id
      });
      const delete_index = raw_snaps.value.findIndex(sp => sp.snap_id === snap_id);
      raw_snaps.value.splice(delete_index, 1);
      // this.raw_snaps.splice(delete_index, 1);
    } catch (e) {
      console.log(e);
    }
  }

  return {
    snapshots,
    FETCH_SNAPSHOTS,
    SAVE_SNAPSHOTS,
    REMOVE_SNAPSHOTS,
  };
});
export default snapStore;
