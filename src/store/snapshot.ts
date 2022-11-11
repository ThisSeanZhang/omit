import { invoke } from '@tauri-apps/api/tauri';
import { sep } from '@tauri-apps/api/path';
import { defineStore } from 'pinia';
import Snapshot from '@/lib/Snapshot';
import { useStore as configStore } from './config'
import { computed, ref } from 'vue';


const SNAPSHOT_FILE = '_snapshots.json';

export const snapStore = defineStore('snap', () => {
  // util
  const config = configStore();

  // const
  const raw_snaps = ref<Snapshot[]>([]);

  // getter
  const snapshots = computed(() => raw_snaps.value);

  // actions
  async function FETCH_SNAPSHOTS(): Promise<void> {
    try {
      // console.log(`${config.userRepoPath}${sep}${SNAPSHOT_FILE}`);
      let snaps_data = await invoke<string>('read_file', {
        filePath: `${config.userRepoPath}${sep}${SNAPSHOT_FILE}`,
      });
      // console.log(`snaps_data: ${snaps_data}`);
      raw_snaps.value = JSON.parse(snaps_data).map(Snapshot.fromObj);
    } catch (e) {
      console.log(e);
    }
  }

  function GET_COPY_WITHOUT(snap_id: string): [Snapshot[], number] {
    const delete_index = raw_snaps.value.findIndex(sp => sp.snap_id === snap_id);
    const copy = raw_snaps.value.slice();
    if (delete_index > -1) copy.splice(delete_index, 1);
    return [copy, delete_index];
  }
  async function  SAVE_SNAPSHOTS(snap: Snapshot): Promise<void> {
    try {
      const [save, index] = GET_COPY_WITHOUT(snap.snap_id);
      console.log(`fined info: ${JSON.stringify(save)}, index: ${index}`);
      if (index > -1) {
        save.splice(index, 0, snap);
      } else {
        save.push(snap);
      }
      await invoke<string>('create_file', {
        dirPath: config.userRepoPath,
        fileName: SNAPSHOT_FILE,
        data: JSON.stringify(save, null, 2)
      });
      // await invoke<string>('save_snapshots', { snaps: JSON.stringify(save, null, 2) });
      raw_snaps.value = save;
    } catch (e) {
      console.log(e);
    }
  }

  async function REMOVE_SNAPSHOTS(snap_id: string): Promise<void> {
    try {
      const [save, _] = GET_COPY_WITHOUT(snap_id);
      // if (index < 0) return;
      // save.splice(index, 1);
      await invoke<string>('create_file', {
        dirPath: config.userRepoPath,
        fileName: SNAPSHOT_FILE,
        data: JSON.stringify(save, null, 2)
      });
      raw_snaps.value = save;
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
