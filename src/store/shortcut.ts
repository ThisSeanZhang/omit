import { invoke } from '@tauri-apps/api/tauri';
import { defineStore } from 'pinia';
import Shortcut from '@/lib/Shortcut';

export const useStore = defineStore('shortcut', {
  state: () => ({
    raw_shortcut: [] as Shortcut[],
  }),
  getters: {
    shortcuts: state => state.raw_shortcut,
  },
  actions: {
    async FETCH_SHORTCURS(): Promise<void> {
      try {
        const msg = await invoke<string>('read_shortcuts');
        console.log(msg);
        console.log(this.raw_shortcut);
        this.raw_shortcut.splice(0, this.raw_shortcut.length);
        console.log(JSON.parse(msg).map(Shortcut.fromObj));
        JSON.parse(msg).map(Shortcut.fromObj).forEach(e => this.raw_shortcut.push(e));
        console.log(this.raw_shortcut);
      } catch (e) {
        console.log(e);
      }
    },
    async SAVE_SHORTCUR(snap: Shortcut): Promise<void> {
      try {
        const save = this.raw_shortcut.slice();
        save.push(snap);
        await invoke<string>('save_shortcuts', { snaps: JSON.stringify(save, null, 2) });
        this.raw_shortcut.push(snap);
      } catch (e) {
        console.log(e);
      }
    },
  },
});
export default useStore;
