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
    GET_COPY_WITHOUT(snap_id: string): [Snapshot[], number] {
      const delete_index = this.raw_snaps.findIndex(sp => sp.snap_id === snap_id);
      const copy = this.raw_snaps.slice();
      if (delete_index > -1) copy.splice(delete_index, 1);
      return [copy, delete_index];
    },
    async SAVE_SNAPSHOTS(snap: Snapshot): Promise<void> {
      try {
        const [save, index] = this.GET_COPY_WITHOUT(snap.snap_id);
        console.log(`fined info: ${JSON.stringify(save)}, index: ${index}`);
        if (index > -1) {
          save.splice(index, 0, snap);
        } else {
          save.push(snap);
        }
        await invoke<string>('save_snapshots', { snaps: JSON.stringify(save, null, 2) });
        this.raw_snaps = save;
      } catch (e) {
        console.log(e);
      }
    },
    async REMOVE_SNAPSHOTS(snap_id: string): Promise<void> {
      try {
        const [save, _] = this.GET_COPY_WITHOUT(snap_id);
        // if (index < 0) return;
        // save.splice(index, 1);
        await invoke<string>('save_snapshots', { snaps: JSON.stringify(save, null, 2) });
        this.raw_snaps = save;
        // this.raw_snaps.splice(delete_index, 1);
      } catch (e) {
        console.log(e);
      }
    },
  },
});
export default useStore;
