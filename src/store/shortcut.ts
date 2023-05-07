import { invoke } from '@tauri-apps/api/tauri';
import { defineStore } from 'pinia';
import Shortcut from '@/lib/Shortcut';
import { useSettingStore } from './setting'
import { computed, ref } from 'vue';

export const shortcutStore = defineStore('shortcut', () => {
  const raw_shortcut = ref<Shortcut[]>([]);
  const settingStore = useSettingStore();

  const shortcuts = computed(() => raw_shortcut.value.slice());

  function SHORTCUT_FORK(): Shortcut[] {
    return raw_shortcut.value.slice();
  }
  async function FETCH_SHORTCURS(): Promise<void> {
    // console.log(`read shortcut path: ${setting.userRepoPath}${sep}${SHORTCUT_FILE}`);
    let shortcut_datas = await invoke<string[]>('read_shortcuts', {
      path: settingStore.setting.shortcut_path
    });
    console.log(shortcut_datas)
    raw_shortcut.value = shortcut_datas.map((e: string) => new Shortcut(JSON.parse(e)));
  }

  async function SAVE_SHORTCUR(snap: Shortcut): Promise<void> {
    try {
      // console.log(`${setting.userRepoPath}${sep}${SHORTCUT_FILE}`);
      const save = raw_shortcut.value.slice();
      save.push(snap);
      await invoke<string>('append_shortcuts', {
        path: settingStore.setting.shortcut_path,
        shortcuts: [[snap.id, JSON.stringify(snap)]]
      });
      raw_shortcut.value = save;
    } catch (e) {
      console.log(e);
    }
  }
  async function SAVE_ALL_SHORTCUR(cuts: Shortcut[]): Promise<void> {
    // try {
    //   await invoke<string>('create_file', {
    //     dirPath: setting.userRepoPath,
    //     fileName: SHORTCUT_FILE,
    //     data: JSON.stringify(cuts, null, 2)
    //   });
    //   raw_shortcut.value = cuts;
    // } catch (e) {
    //   console.log(e);
    // }
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
