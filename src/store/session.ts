import { invoke } from '@tauri-apps/api/tauri';
import { defineStore } from 'pinia';
import { useSettingStore } from './setting'
import { computed, ref } from 'vue';
import OmitSession from '@/lib/OmitSession';

export const useSessionStore = defineStore('session', () => {
  const raw_sessions = ref<OmitSession[]>([]);
  const settingStore = useSettingStore();

  const sessions = computed(() => raw_sessions.value);

  async function FETCH_SESSIONS(): Promise<void> {
    let result = await invoke<string[]>('read_sesssions', {
      path: settingStore.setting.sessions_path,
    });
    raw_sessions.value = result
      .map(session_data => JSON.parse(session_data))
      .map((e: any) => new OmitSession(e));
  }

  function FIND_SESSION_INFO(name: String): OmitSession | undefined {
    let found = raw_sessions.value.find(sess => sess.name === name);
    return found;
  }

  async function SAVE_SESSION(session: OmitSession): Promise<void> {
    await invoke('append_sesssions', {
      path: settingStore.setting.sessions_path,
      sesssions: [[session.id, JSON.stringify(session)]]
    });
    raw_sessions.value = raw_sessions.value.filter(sess => sess.name !== session.name);
    raw_sessions.value.push(session);
  }

  async function DELETE_SESSION(session_id: string): Promise<void> {
    await invoke('delete_sesssion', {
      path: settingStore.setting.sessions_path,
      id: session_id
    });
    raw_sessions.value = raw_sessions.value.filter(sess => sess.id !== session_id);
  }

  return {
    sessions,
    FETCH_SESSIONS,
    FIND_SESSION_INFO,
    SAVE_SESSION,
    DELETE_SESSION,
  }
});
export default useSessionStore;
