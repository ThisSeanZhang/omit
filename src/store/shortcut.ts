import { invoke } from '@tauri-apps/api/tauri';
import { defineStore } from 'pinia';
import Shortcut from '@/lib/Shortcut';
import { useStore as configStore } from './config'
import { computed, ref } from 'vue';

const SHORTCUT_FILE = '_shortcuts.json';

export const shortcutStore = defineStore('shortcut', () => {
  const raw_shortcut = ref<Shortcut[]>([]);
  const config = configStore();

  const shortcuts = computed(() => raw_shortcut.value);

  async function FETCH_SHORTCURS(): Promise<void> {
    let setting_data = await invoke<string>('read_file', {
      filePath: config.userRepoPath,
      fileName: SHORTCUT_FILE
    });
    raw_shortcut.value = JSON.parse(setting_data).map(Shortcut.fromObj);
  }

  async function SAVE_SHORTCUR(snap: Shortcut): Promise<void> {
    try {
      const save = raw_shortcut.value.slice();
      save.push(snap);
      await invoke<string>('create_file', {
        dirPath: config.userRepoPath,
        fileName: SHORTCUT_FILE,
        data: JSON.stringify(save, null, 2)
      });
      raw_shortcut.value.push(snap);
    } catch (e) {
      console.log(e);
    }
  }
  async function SAVE_ALL_SHORTCUR(): Promise<void> {
    try {
      await invoke<string>('create_file', {
        dirPath: config.userRepoPath,
        fileName: SHORTCUT_FILE,
        data: JSON.stringify(raw_shortcut.value, null, 2)
      });
    } catch (e) {
      console.log(e);
    }
  }

  return {
    shortcuts,
    FETCH_SHORTCURS,
    SAVE_SHORTCUR,
    SAVE_ALL_SHORTCUR,
  }
});
export default shortcutStore;
