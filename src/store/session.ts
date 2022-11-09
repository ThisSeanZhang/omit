import { invoke } from '@tauri-apps/api/tauri';
import { sep } from '@tauri-apps/api/path';
import { defineStore } from 'pinia';
import { useStore as configStore } from './config'
import { computed, ref } from 'vue';
import OmitSession from '@/lib/OmitSession';

export const sessionStore = defineStore('session', () => {
  const raw_sessions = ref<OmitSession[]>([]);
  const config = configStore();

  const sessions = computed(() => raw_sessions.value);

  async function FETCH_SESSIONS(): Promise<void> {
    let sessions_files = await invoke<string[]>('list_dir_all', {
      dirPath: config.sessionsPath,
    });
    const result = await Promise.all(
      sessions_files.map(file_name => 
        (async () => await invoke<string>('read_file', { filePath: `${config.sessionsPath}${sep}${file_name}` }))()
      )
    );
    result.map(session_data => JSON.parse(session_data))
      .map(OmitSession.fromObj)
      .forEach(session => raw_sessions.value.push(session));
  }

  async function SAVE_SESSION(session: OmitSession): Promise<void> {
    const data = JSON.stringify(session, null, 2);
    await invoke('create_file', {
      dirPath: config.sessionsPath,
      fileName: `${session.name}.json`,
      data
    });
    raw_sessions.value = raw_sessions.value.filter(sess => sess.name !== session.name);
    raw_sessions.value.push(session);
  }

  return {
    sessions,
    FETCH_SESSIONS,
    SAVE_SESSION,
  }
});
export default sessionStore;
