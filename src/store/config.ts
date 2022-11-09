import { invoke } from '@tauri-apps/api/tauri';
import { homeDir, sep } from '@tauri-apps/api/path';
// import { exists, createDir, BaseDirectory  } from '@tauri-apps/api/fs';
import { defineStore } from 'pinia';
import Repo from '@/lib/Repo';

const USER_HOME_CONFIG_DIR_NAME = 'omit';
const SETTINGS_FILE_NAME = 'settings.json';
const REPOS_FOLDER = 'repos';
const SESSIONS_FOLDER_NAME = 'sessions';
const SNAPSHOT_FILE = '_snapshots.json';

console.log(sep)
const USER_HOME_PATH = await homeDir();
const APP_CONFIG_HOME = `${USER_HOME_PATH}${USER_HOME_CONFIG_DIR_NAME}`;

export const useStore = defineStore('config', {
  state: () => ({
    app_config_home: APP_CONFIG_HOME,
    config_file: `${APP_CONFIG_HOME}${sep}${SETTINGS_FILE_NAME}`,
    repos_folder: `${APP_CONFIG_HOME}${sep}${REPOS_FOLDER}`,
    sessions_folder: `${APP_CONFIG_HOME}${sep}${SESSIONS_FOLDER_NAME}`,
    user_repo_name: '',
  }),
  getters: {
    configPath: state => state.config_file,
    reposPath: state => state.repos_folder,
    sessionsPath: state => state.sessions_folder,
    userRepoName: state => state.user_repo_name,
    userRepoPath: state => `${state.repos_folder}${sep}${state.user_repo_name}`,
    getConfigClone: state => new Config(state.repos_folder, state.sessions_folder, state.user_repo_name),
  },
  actions: {
    async CHECK_SETTING_FILE(): Promise<Boolean> {
      try {
        let setting_data = await invoke<string>('read_file', {
          filePath: this.configPath
        });
        let setting:any = JSON.parse(setting_data);
        console.log(setting);
        this.repos_folder = setting.repos_folder;
        this.sessions_folder = setting.sessions_folder;
        this.user_repo_name = setting.user_repo_name;
      } catch (error) {
        return false;
      }
      // const result = await invoke<Boolean>('create_file', {
      //   dirPath: this.app_config_home,
      // });
      return true;
    },
    async REWRITE_CONFIG(config: Config): Promise<void> {
      try {
        this.repos_folder = config.repos_folder;
        this.sessions_folder = config.sessions_folder;
        this.user_repo_name = config.user_repo_name;
        const data = JSON.stringify(config, null, 2);
        await invoke('create_file', {
          dirPath: this.app_config_home,
          fileName: SETTINGS_FILE_NAME,
          data
        });

        await invoke('create_file', {
          dirPath: this.repos_folder,
        });

        await invoke('create_file', {
          dirPath: this.sessions_folder,
        });

        await invoke('create_file', {
          dirPath: `${this.repos_folder}${sep}${this.user_repo_name}`,
        });
      } catch (e) {
        console.log(e);
      }
    },
  },
});
export class Config {
  repos_folder: string;
  sessions_folder: string;
  user_repo_name: string;

  constructor(repos_folder: string = '', sessions_folder: string= '', user_repo_name: string= '') {
    this.repos_folder = repos_folder;
    this.sessions_folder = sessions_folder;
    this.user_repo_name = user_repo_name;
  }


}
export default useStore;
