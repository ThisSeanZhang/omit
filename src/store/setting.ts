import { invoke } from '@tauri-apps/api/tauri';
import { ref, computed } from 'vue';
import { defineStore } from 'pinia';
import { Setting } from '@/lib/setting';

export const useSettingStore = defineStore('config', () => {
  const _setting = ref<Setting>(new Setting({}));

  const setting = computed(() => _setting.value);
  // const getSettingClone = computed(() => _setting.value.fork());

  async function FETCH_SYSTEM_SETTING() {
    try {
      let setting_data = await invoke<string>('read_setting');

      let setting: Setting = JSON.parse(setting_data);
      _setting.value = new Setting(setting);
    } catch (error) {
      
    }
  }

  async function SAVE_SETTING(setting: Setting): Promise<void> {
    try {
      await invoke('save_setting', {
        setting: JSON.stringify(setting, null, 2)
      });

      _setting.value = setting;
    } catch (e) {
      console.log(e);
    }
  }

  return {
    setting,
    FETCH_SYSTEM_SETTING,
    SAVE_SETTING,
  };
});


export default useSettingStore;
