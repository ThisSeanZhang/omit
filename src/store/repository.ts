import { invoke } from '@tauri-apps/api/tauri';
import { sep } from '@tauri-apps/api/path';
import { defineStore } from 'pinia';
import Repo from '@/lib/Repo';
import Command from '@/lib/Command';
import { useSettingStore } from './setting'
import { computed, ref } from 'vue';

export const repoStore = defineStore('repositpry', () => {
  // util
  const settingStore = useSettingStore();

  // const
  const raw_repo_dirs = ref<string[]>([]);
  const raw_repo = ref<Repo[]>([]);

  // getter
  const repo_dirs = computed(() => raw_repo.value);
  const repos = computed(() => raw_repo.value);

  async function FETCH_REPOS(): Promise<void> {
    try {
      const repo_dirs = await invoke<string[]>('list_dir_all', {
        dirPath: settingStore.setting.command_path,
      });
      raw_repo.value = await Promise.all(
        repo_dirs.map(repo_name => 
            (async () => await read_single_repo(repo_name))()
          )
        );
    } catch (e) {
      console.log(e);
    }
  }

  async function read_single_repo(repo_name: string): Promise<Repo> {
    // console.log(`read repo: ${repo_name}`);
    const repo_path = `${settingStore.setting.command_path}${sep}${repo_name}`;
    const repo = new Repo(repo_name);
    try {
      const repo_dir_list = await invoke<string[]>('list_dir_only_folder', {
        dirPath: `${settingStore.setting.command_path}${sep}${repo_name}`,
      });
      
      const repo_cmd_dirs = repo_dir_list
        // .filter(file_name => file_name !== "LICENSE")
        // .filter(file_name => file_name !== "README.md")
        .filter(file_name => !file_name.endsWith('.json'))
        .filter(file_name => !file_name.startsWith('.'));
      
      // console.log("in repo cmds: ");
      // console.log(repo_cmd_dirs);
      const cmd_datas = await Promise.all(
        repo_cmd_dirs.map(cmd_dir_name => 
          (async () => {
            const cmd_dir_path = `${repo_path}${sep}${cmd_dir_name}`
            const cmd_files = await invoke<string[]>('list_dir_all', { dirPath: cmd_dir_path });
            // console.log("read cmd: ");
            // console.log(cmd_files);
            return await Promise.all(
              cmd_files.map(cmd_file_name => 
                (async () => await invoke<string>('read_file', { filePath: `${cmd_dir_path}${sep}${cmd_file_name}` }))()
              )
            )
          })()
        )
      );
      // console.log(cmd_datas);
      const cmds = cmd_datas
        .flatMap(cmd_datas => cmd_datas.map(cmd_data => JSON.parse(cmd_data)))
        // .map(cmd_string => {console.log(cmd_string); return cmd_string;})
        .map(Command.fromObj)
        .map(cmd => {
          // TODO maybe useless
          cmd.belong_file = '';
          cmd.belong_repo = repo_name;
          cmd.command_id = cmd.getCommandId();
          return cmd;
        });
      repo.commands = cmds;
    } catch (error) {
      console.log(error);
    }
    return repo;
  }

  
  // async function FETCH_REPO_CMDS(name: string): Promise<void> {
  //   console.log(`read cmd: ${name}`);
  //   try {
  //     const msg = await invoke<any>('read_repo_command', { repoDir: name });
  //     console.log(msg);
  //     this.raw_repo.splice(0, this.raw_repo.length);
  //     this.raw_repo.push(new Repo(name, msg));
  //     console.log(this.raw_repo);
  //   } catch (e) {
  //     console.log(e);
  //   }
  // }

  function FIND_CMD_USE(cmd_id: string): Command {
    const split_keys = cmd_id.split('|');
    const repo = raw_repo.value.find(e => e.name === split_keys[0]);
    if (repo === undefined) return null as unknown as Command;

    const cmd = repo.commands.find(e => e.command_name === split_keys[1]);
    if (cmd === undefined) return null as unknown as Command;

    return cmd;
  }
  
  return {
    repo_dirs,
    repos,
    FETCH_REPOS,
    FIND_CMD_USE,
  };
});
export default repoStore;
