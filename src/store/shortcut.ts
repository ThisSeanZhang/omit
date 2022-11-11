import { invoke } from '@tauri-apps/api/tauri';
import { sep } from '@tauri-apps/api/path';
import { defineStore } from 'pinia';
import Shortcut from '@/lib/Shortcut';
import { useStore as configStore } from './config'
import { computed, ref } from 'vue';

const SHORTCUT_FILE = '_shortcuts.json';

export const shortcutStore = defineStore('shortcut', () => {
  const raw_shortcut = ref<Shortcut[]>([]);
  const config = configStore();

  const shortcuts = computed(() => raw_shortcut.value.slice());

  function SHORTCUT_FORK(): Shortcut[] {
    return raw_shortcut.value.slice();
  }
  async function FETCH_SHORTCURS(): Promise<void> {
    // console.log(`read shortcut path: ${config.userRepoPath}${sep}${SHORTCUT_FILE}`);
    let shortcut_datas = await invoke<string>('read_file', {
      filePath: `${config.userRepoPath}${sep}${SHORTCUT_FILE}`,
    });
    raw_shortcut.value = JSON.parse(shortcut_datas).map(Shortcut.fromObj);
  }

  async function SAVE_SHORTCUR(snap: Shortcut): Promise<void> {
    try {
      // console.log(`${config.userRepoPath}${sep}${SHORTCUT_FILE}`);
      const save = raw_shortcut.value.slice();
      save.push(snap);
      await invoke<string>('create_file', {
        dirPath: config.userRepoPath,
        fileName: SHORTCUT_FILE,
        data: JSON.stringify(save, null, 2)
      });
      raw_shortcut.value = save;
    } catch (e) {
      console.log(e);
    }
  }
  async function SAVE_ALL_SHORTCUR(cuts: Shortcut[]): Promise<void> {
    try {
      await invoke<string>('create_file', {
        dirPath: config.userRepoPath,
        fileName: SHORTCUT_FILE,
        data: JSON.stringify(cuts, null, 2)
      });
      raw_shortcut.value = cuts;
    } catch (e) {
      console.log(e);
    }
  }

  return {
    shortcuts,
    SHORTCUT_FORK,
    FETCH_SHORTCURS,
    SAVE_SHORTCUR,
    SAVE_ALL_SHORTCUR,
  }
});
export default shortcutStore;
