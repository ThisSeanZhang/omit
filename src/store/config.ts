import { invoke } from '@tauri-apps/api/tauri';
import { homeDir, sep } from '@tauri-apps/api/path';
import { ref, computed } from 'vue';
import { defineStore } from 'pinia';

// immutable setting folder and file name
const USER_HOME_CONFIG_DIR_NAME = 'omit';
const SETTINGS_FILE_NAME = 'settings.json';

const USER_HOME_PATH = await homeDir();
const APP_CONFIG_HOME = `${USER_HOME_PATH}${USER_HOME_CONFIG_DIR_NAME}`;
const APP_CONFIG_FILE = `${APP_CONFIG_HOME}${sep}${SETTINGS_FILE_NAME}`;

// mutable repos path and sessions path
const REPOS_FOLDER = 'repos';
const SESSIONS_FOLDER_NAME = 'sessions';

export class Config {
  repos_folder: string;
  sessions_folder: string;
  user_repo_name: string;

  constructor() {
    this.repos_folder = `${APP_CONFIG_HOME}${sep}${REPOS_FOLDER}`;
    this.sessions_folder = `${APP_CONFIG_HOME}${sep}${SESSIONS_FOLDER_NAME}`;
    this.user_repo_name = '';
  }

  fork(): Config {
    const config = new Config();
    config.repos_folder = this.repos_folder;
    config.sessions_folder = this.sessions_folder;
    config.user_repo_name = this.user_repo_name;
    return config;
  }

}

async function FETCH_SYSTEM_CONFIG():Promise<[Config, Boolean]> {
  try {
    let setting_data = await invoke<string>('read_file', {
      filePath: APP_CONFIG_FILE
    });
    let config: Config = JSON.parse(setting_data);
    return [config, false];
  } catch (error) {
    return [new Config(), true];
  }
}
const [init_config, is_default] = await FETCH_SYSTEM_CONFIG();

export const useStore = defineStore('config', () => {
  const raw_config = ref<Config>(init_config);
  const init_error = ref(is_default);

  const needToInit = computed(() => init_error.value || raw_config.value.user_repo_name === '');
  const reposPath = computed(() => raw_config.value.repos_folder);
  const sessionsPath = computed(() => raw_config.value.sessions_folder);
  const userRepoName = computed(() => raw_config.value.user_repo_name);
  const userRepoPath = computed(() => `${raw_config.value.repos_folder}${sep}${raw_config.value.user_repo_name}`);
  // const getConfigClone = computed(() => raw_config.value.fork());

  function GET_CONFIG_FORK():Config {
    const config = new Config();
    config.repos_folder = raw_config.value.repos_folder;
    config.sessions_folder = raw_config.value.sessions_folder;
    config.user_repo_name = raw_config.value.user_repo_name;
    return config;
  }
  async function REWRITE_CONFIG(config: Config): Promise<void> {
    try {
      const data = JSON.stringify(config, null, 2);
      await invoke('create_file', {
        dirPath: APP_CONFIG_HOME,
        fileName: SETTINGS_FILE_NAME,
        data
      });

      await invoke('create_file', {
        dirPath: config.repos_folder,
      });

      await invoke('create_file', {
        dirPath: config.sessions_folder,
      });

      await invoke('create_file', {
        dirPath: `${config.repos_folder}${sep}${config.user_repo_name}`,
      });
      raw_config.value = config;
    } catch (e) {
      console.log(e);
    }
  }
  return {
    needToInit,
    reposPath,
    sessionsPath,
    userRepoName,
    userRepoPath,
    // getConfigClone,
    GET_CONFIG_FORK,
    REWRITE_CONFIG,
  };
});


export default useStore;
