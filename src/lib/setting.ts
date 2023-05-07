import { homeDir, sep } from '@tauri-apps/api/path';

export const CURRENT_VERSION = 2;

const OMIT_BASE_PATH = ".omit"

// immutable setting folder and file name
const USER_HOME_PATH = await homeDir();
const USER_HOME_DAFAULT_DIR_NAME = `${USER_HOME_PATH}${sep}${OMIT_BASE_PATH}`;
const DATA_PATH = 'data';


// mutable repos path and sessions path
// const REPOS_FOLDER = 'repos';
const SESSIONS_FOLDER_NAME = 'sessions';
const SNAP_FOLDER_NAME     = 'snapshots';
const SHORTCUT_FOLDER_NAME = 'shortcut';
const COMMAND_FOLDER_NAME  = 'command';

export class Setting {
  version: number;
  data_base_path: string;
  sessions_path: string;
  snapshots_path: string;
  shortcut_path: string;
  command_path: string

  // repos_folder: string;
  user_repo_name: string;

  constructor(in_config: {
    version?: number
    data_base_path?: string;
    user_repo_name?: string
    snapshots_path?: string;
    shortcut_path?: string;
    command_path?: string


    // repos_folder?: string
    sessions_path?: string
  }) {
    this.version = in_config.version ?? CURRENT_VERSION;
    this.data_base_path = in_config.data_base_path ?? `${USER_HOME_DAFAULT_DIR_NAME}${sep}${DATA_PATH}`

    this.sessions_path = in_config.sessions_path ??`${USER_HOME_DAFAULT_DIR_NAME}${sep}${SESSIONS_FOLDER_NAME}`;
    this.snapshots_path = in_config.snapshots_path ??`${USER_HOME_DAFAULT_DIR_NAME}${sep}${SNAP_FOLDER_NAME}`;
    this.shortcut_path = in_config.shortcut_path ??`${USER_HOME_DAFAULT_DIR_NAME}${sep}${SHORTCUT_FOLDER_NAME}`;
    this.command_path = in_config.command_path ??`${USER_HOME_DAFAULT_DIR_NAME}${sep}${COMMAND_FOLDER_NAME}`;
    
    // this.repos_folder = in_config.repos_folder ?? `${USER_HOME_DAFAULT_DIR_NAME}${sep}${REPOS_FOLDER}`;
    this.user_repo_name = in_config.user_repo_name ?? '';
  }

  set base_path(base_path: string) {
    this.data_base_path = base_path
    this.sessions_path = `${base_path}${sep}${SESSIONS_FOLDER_NAME}`;
    this.snapshots_path = `${base_path}${sep}${SNAP_FOLDER_NAME}`;
    this.shortcut_path = `${base_path}${sep}${SHORTCUT_FOLDER_NAME}`;
    this.command_path = `${base_path}${sep}${COMMAND_FOLDER_NAME}`;
  }

}