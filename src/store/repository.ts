// import { invoke } from '@tauri-apps/api/tauri';
// import { defineStore } from 'pinia';
// import Repo from '@/lib/Repo';

// export const useStore = defineStore('repositpry', {
//   state: () => ({
//     repo_dirs: [] as string[],
//     repos: [] as Repo[],
//   }),
//   getters: {
//     repo_dirs: state => state.repo_dirs,
//     repos: state => state.repos,
//   },
//   actions: {
//     async FETCH_REPOS(): Promise<string[]> {
//       try {
//         // invoke<string[]>('get_repo_dirs')
//         //   .then(msg => {
//         //     this.repo_dirs = msg;
//         //   }).catch((msg:string) => console.log(msg));
//         return await invoke<string[]>('get_repo_dirs');
//       } catch (e) {
//         console.log(e);
//         return [];
//       }
//     },
//     async FETCH_REPO_CMDS(name: string): Promise<void> {
//       try {
//         const msg = await invoke<Map<string, string>>('read_repo_command', { repoDir: name });
//         this.repos.push(new Repo(name, msg));
//         console.log(this.repos);
//       } catch (e) {
//         console.log(e);
//       }
//     },
//   },
// });
// export default useStore;
