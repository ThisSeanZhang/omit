import { invoke } from '@tauri-apps/api/tauri';
import { defineStore } from 'pinia';
import Snapshot from '@/lib/Snapshot';

export const useStore = defineStore('snap', {
  state: () => ({
    raw_snaps: [] as Snapshot[],
  }),
  getters: {
    snapshots: state => state.raw_snaps,
  },
  actions: {
    async FETCH_SNAPSHOTS(): Promise<void> {
      try {
        const msg = await invoke<string>('read_snapshots');
        // console.log(msg);
        // console.log(this.raw_snaps);
        this.raw_snaps.splice(0, this.raw_snaps.length);
        // console.log(JSON.parse(msg).map(Snapshot.fromObj));
        JSON.parse(msg).map(Snapshot.fromObj).forEach(e => this.raw_snaps.push(e));
        // console.log(this.raw_snaps);
      } catch (e) {
        console.log(e);
      }
    },
    async SAVE_SNAPSHOTS(snap: Snapshot): Promise<void> {
      try {
        const save = this.raw_snaps.slice();
        save.push(snap);
        await invoke<string>('save_snapshots', { snaps: JSON.stringify(save, null, 2) });
        this.raw_snaps.push(snap);
      } catch (e) {
        console.log(e);
      }
    },
    async REMOVE_SNAPSHOTS(snap_id: string): Promise<void> {
      try {
        const delete_index = this.raw_snaps.findIndex(sp => sp.snap_id === snap_id);
        if (delete_index < 0) return;
        const save = this.raw_snaps.splice(delete_index, 1);
        await invoke<string>('save_snapshots', { snaps: JSON.stringify(save, null, 2) });
        this.raw_snaps.splice(delete_index, 1);
      } catch (e) {
        console.log(e);
      }
    },
  },
});
export default useStore;
